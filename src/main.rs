extern crate image;

use std::env;
use std::io;
use std::io::Write;
use std::path::Path;

use image::GenericImage;

type Pixel = image::Rgba<u8>;
type Container = Vec<u8>;

fn split_rgb(rgba: &[u8; 4]) -> (Pixel, Pixel, Pixel) {
    let r = image::Rgba{
        data: [rgba[0],0,0,rgba[3]],
    };
    let g = image::Rgba{
        data: [0,rgba[1],0,rgba[3]],
    };
    let b = image::Rgba{
        data: [0,0,rgba[2],rgba[3]],
    };

    (r,g,b)
}


trait Pattern {
    fn size(&self) -> u32;
    fn slice(&self, x: u32, y: u32, rgb: &[u8; 4], target: &mut image::ImageBuffer<Pixel, Container>);
}

// Most common pattern: simple RGB
struct RGB;
impl Pattern for RGB {
    fn size(&self) -> u32 {
        3
    }

    fn slice(&self, x: u32, y: u32, rgb: &[u8; 4], target: &mut image::ImageBuffer<Pixel, Container>) {

        let (r,g,b) = split_rgb(rgb);

        // We return a square, three times the same row: r,g,b
        for j in 0..3 {
            target[(3*x+0, 3*y+j)] = r;
            target[(3*x+1, 3*y+j)] = g;
            target[(3*x+2, 3*y+j)] = b;
        }
    }
}

// Pentile RGBW pattern, found on some hidpi screens
struct RGBW;
impl Pattern for RGBW {
    fn size(&self) -> u32 {
        2
    }

    fn slice(&self, x: u32, y: u32, rgb: &[u8; 4], target: &mut image::ImageBuffer<Pixel, Container>) {
        let short = (x+y)%2 == 1;
        if short {
            // Get brightness from the pixel?
            let brightness = rgb[0]/3+rgb[1]/3+rgb[2]/3;
            let w = image::Rgba{
                data: [brightness, brightness, brightness, rgb[3]],
            };
            target[(2*x+1,2*y+0)] = w;
            target[(2*x+1,2*y+1)] = w;
        } else {
            let (r,g,b) = split_rgb(rgb);
            for j in 0..2 {
                target[(2*x+0,2*y+j)] = r;
                target[(2*x+1,2*y+j)] = g;
                if 2*x+2 < target.width() {
                    target[(2*x+2,2*y+j)] = b;
                }
            }
        }
    }
}

fn parse_pattern(string: &str) -> Box<Pattern> {
    match string {
        "RGB" | "rgb" => Box::new(RGB),
        "RGBW" | "rgbw" => Box::new(RGBW),
        _ => panic!("Unrecognized pattern: {}", string),
    }
}

fn main() {
    // Skip exe name
    let mut args = env::args();
    let binary = args.next().unwrap();

    // First mandatory argument is the input image filename
    let filename = match args.next() {
        None => {
            panic!("No file given. Usage: {} FILE [RGB|RGBW]", binary);
        },
        Some(text) => text,
    };

    // Second optionnal argument
    let pattern: Box<Pattern> = match args.next() {
        None => Box::new(RGB),
        Some(text) => parse_pattern(&text),
    };

    println!("Loading file {}", filename);
    let source = image::open(&Path::new(&filename)).unwrap();

    let (w,h) = source.dimensions();

    // This is a simple progress indicator: show dots.
    let max_dots = 20;
    let dot_every = (w / max_dots * h) as usize;

    // Scale up
    let size = pattern.size();
    let w = w * size;
    let h = h * size;


    println!("Creating blank canvas: {} x {}", w, h);
    let mut target = image::ImageBuffer::new(w, h);

    println!("Iterating");
    for (k,(x,y,pixel)) in source.pixels().enumerate() {

        // result is a size*size square of pixels, in row-major.
        pattern.slice(x, y, &pixel.data, &mut target);

        // Trivial progress indicator
        if k % dot_every == 0 {
            print!(".");
            io::stdout().flush().expect("Could not flush?!?");
        }
    }
    println!("");

    // Aaand we're done.
    let target_filename = format!("{}.subs.png", filename);
    println!("Now saving image as {}", target_filename);

    let _ = target.save(&target_filename).unwrap();
}
