//! image-stitcher
//! image-stitcher
//! image-stitcher
//! image-stitcher

// use clap::Parser;
use image::{GenericImageView, ImageBuffer};

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

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let imgx = image::open("/home/chum/repos/dotfiles/prose/screenshot1.png").unwrap();
    let imgy = image::open("/home/chum/repos/dotfiles/prose/screenshot2.png").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", imgx.dimensions());
    println!("dimensions {:?}", imgy.dimensions());

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx.width(), imgx.height() + imgy.height());

    // Iterate over the coordinates and pixels of the image
    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //     let r = (0.3 * x as f32) as u8;
    //     let b = (0.3 * y as f32) as u8;
    //     *pixel = image::Rgb([r, 0, b]);
    // }
    for (x, y, pixel) in imgx.pixels() {
        imgbuf.put_pixel(x, y, pixel);
    }
    for (x, y, pixel) in imgy.pixels() {
        imgbuf.put_pixel(x, y + imgx.height(), pixel);
    }

    // Write the contents of this image to the Writer in PNG format.
    imgbuf.save("test.png").unwrap();
}
