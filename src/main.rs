extern crate image;

use std::env;
use std::path::Path;

use image::GenericImage;

trait Pattern {
    fn size(&self) -> u32;
    fn slice(&self, rgb: &[u8; 4]) -> Vec<image::Rgba<u8>>;
}

// Currently only one pattern is implemented: RGB
struct RGB;
impl Pattern for RGB {
    fn size(&self) -> u32 {
        3
    }

    fn slice(&self, rgb: &[u8; 4]) -> Vec<image::Rgba<u8>> {
        let r = image::Rgba{
            data: [rgb[0],0,0,rgb[3]],
        };
        let g = image::Rgba{
            data: [0,rgb[1],0,rgb[3]],
        };
        let b = image::Rgba{
            data: [0,0,rgb[2],rgb[3]],
        };

        // We return a square, three times the same row: r,g,b
        vec![r,g,b,
             r,g,b,
             r,g,b]
    }
}

fn parse_pattern(string: &str) -> Box<Pattern> {
    Box::new(match string {
        "RGB" | "rgb" => RGB,
        _ => panic!("Unrecognized pattern: {}", string),
    })
}

fn main() {
    // Skip exe name
    let mut args = env::args();
    let binary = args.next().unwrap();

    // First mandatory argument is the input image filename
    let filename = match args.next() {
        None => {
            panic!("No file given. Usage: {} FILE", binary);
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

        // Scale from source to target coordinates
        let x = x * size;
        let y = y * size;

        // result is a size*size square of pixels, in row-major.
        let result = pattern.slice(&pixel.data);

        // Blitz it into the target image.
        for i in 0..size {
            let x = x+i;
            for j in 0..size {
                let y = y+j;

                target[(x,y)] = result[(i + size * j) as usize];
            }
        }

        // Trivial progress indicator
        if k % dot_every == 0 {
            print!(".");
        }
    }
    println!("");

    // Aaand we're done.
    let target_filename = format!("{}.subs.png", filename);
    println!("Now saving image as {}", target_filename);

    let _ = target.save(&target_filename).unwrap();
}
