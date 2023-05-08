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
    let orient = &cli.orient;

    let imgx = image::open(cli.image_1).unwrap();
    let imgy = image::open(cli.image_2).unwrap();

    println!("IMAGE_1 dimensions {:?}", imgx.dimensions());
    println!("IMAGE_2 dimensions {:?}", imgy.dimensions());

    let mut imgbuf;

    if orient.horizontal {
        imgbuf = image::ImageBuffer::new(imgx.width() + imgy.width(), imgx.height());
        imgbuf.copy_from(&imgx, 0, 0).unwrap();
        imgbuf.copy_from(&imgy, imgx.width(), 0).unwrap();
    } else if orient.vertical {
        imgbuf = image::ImageBuffer::new(imgx.width(), imgx.height() + imgy.height());
        imgbuf.copy_from(&imgx, 0, 0).unwrap();
        imgbuf.copy_from(&imgy, 0, imgx.height()).unwrap();
    } else {
        panic!("`orient` did not contain `horizontal` or `vertical`");
    }

    println!("output image dimensions = {:?}", imgbuf.dimensions());
    imgbuf.save("test.png").unwrap();
}
