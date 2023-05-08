//! image-stitcher
//! image-stitcher
//! image-stitcher
//! image-stitcher

use clap::{Args, Parser};
use image::{GenericImage, GenericImageView};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    orient: Orient,

    #[arg(required = true)]
    image_1: String,

    #[arg(required = true)]
    image_2: String,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Orient {
    #[arg(long)]
    horizontal: bool,

    #[arg(long)]
    vertical: bool,
}

fn main() {
    let cli = Cli::parse();

    let imgx = image::open(cli.image_1).unwrap();
    let imgy = image::open(cli.image_2).unwrap();

    println!("dimensions {:?}", imgx.dimensions());
    println!("dimensions {:?}", imgy.dimensions());

    let mut imgbuf = image::ImageBuffer::new(imgx.width(), imgx.height() + imgy.height());
    imgbuf.copy_from(&imgx, 0, 0).unwrap();
    imgbuf.copy_from(&imgy, 0, imgx.height()).unwrap();
    println!("output image dimensions = {:?}", imgbuf.dimensions());
    imgbuf.save("test.png").unwrap();
}
