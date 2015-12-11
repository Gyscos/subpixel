extern crate image;

use std::env;
use std::path::Path;

use image::GenericImage;

enum Pattern {
    RGB,
}

impl Pattern {
    fn new(string: &str) -> Self {
        match string {
            "rgb" | "RGB" => Pattern::RGB,
            _ => panic!("Unrecognized pattern"),
        }
    }

    fn size(&self) -> u32 {
        match self {
            &Pattern::RGB => 3,
        }
    }

    fn slice(&self, rgb: &[u8; 4]) -> Vec<image::Rgba<u8>> {
        match self {
            &Pattern::RGB => {
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
            },
        }
    }

}

fn main() {
    // Skip exe name
    let mut args = env::args();
    args.next().unwrap();
    let filename = match args.next() {
        None => {
            panic!("No file given");
        },
        Some(text) => { println!("Loading {}", text); text },
    };

    let pattern = match args.next() {
        None => Pattern::RGB,
        Some(text) => { println!("Parsing {}", text); Pattern::new(&text) },
    };

    println!("Loading file...");
    let source = image::open(&Path::new(&filename)).unwrap();

    let (w,h) = source.dimensions();

    let max_dots = 20;
    let dot_every = (w / max_dots * h) as usize;

    // Scale up
    let size = pattern.size();
    let w = w * size;
    let h = h * size;


    println!("Creating blank canvas");
    let mut target = image::ImageBuffer::new(w, h);

    println!("Iterating...");
    for (k,(x,y,pixel)) in source.pixels().enumerate() {

        let x = x * size;
        let y = y * size;

        let result = pattern.slice(&pixel.data);

        for i in 0..size {
            let x = x+i;
            for j in 0..size {
                let y = y+j;

                target[(x,y)] = result[(i + size * j) as usize];
            }
        }

        if k % dot_every == 0 {
            print!(".");
        }
    }
    println!("");
    let target_filename = format!("{}.subs.png", filename);
    println!("Now saving image as {}...", target_filename);

    let _ = target.save(&target_filename).unwrap();
}
