//! image-stitcher
//! image-stitcher
//! image-stitcher
//! image-stitcher

// use clap::Parser;
use image::{GenericImage, GenericImageView};

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Name of the person to greet
//     #[arg(short, long)]
//     name: String,
//
//     /// Number of times to greet
//     #[arg(short, long, default_value_t = 1)]
//     count: u8,
// }

fn main() {
    // let args = Args::parse();

    let imgx = image::open("/home/chum/repos/dotfiles/prose/screenshot1.png").unwrap();
    let imgy = image::open("/home/chum/repos/dotfiles/prose/screenshot2.png").unwrap();

    println!("dimensions {:?}", imgx.dimensions());
    println!("dimensions {:?}", imgy.dimensions());

    let mut imgbuf = image::ImageBuffer::new(imgx.width(), imgx.height() + imgy.height());
    imgbuf.copy_from(&imgx, 0, 0).unwrap();
    imgbuf.copy_from(&imgy, 0, imgx.height()).unwrap();
    println!("output image dimensions = {:?}", imgbuf.dimensions());
    imgbuf.save("test.png").unwrap();
}
