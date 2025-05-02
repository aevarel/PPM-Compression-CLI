use crate::image::{image_trim, read_image_data};
use crate::rgb_cvc;
use crate::cvc_block;
use crate::blocks_bwc;
use csc411_image::{RgbImage, Read, Write};

/// Compresses the image file
/// 
/// # Arguments
/// 
/// * 'filename' - an optional filename of type Option<&String>
pub fn compress(filename: Option<&str>) {
    let mut image: RgbImage = RgbImage::read(filename).unwrap();
    image_trim(&mut image);

    let rgb_pixels = image.pixels;
    let rgbf64_pixels = rgb_cvc::rgb_to_rgbf64(rgb_pixels.clone());
    let cvc_pixels = rgb_cvc::rgbf64_to_component_video(rgbf64_pixels.clone());
    let cvc_blocks = cvc_block::cvc_to_blocks(cvc_pixels.clone(), image.width as usize, image.height as usize);
    let bwc_words = blocks_bwc::block_to_bwc(cvc_blocks.clone());
    
    // Write to stdout  
    println!("Compressed image format 2\n{} {}", image.width, image.height);
    for word in bwc_words.words {
        println!("{}", word);
        }
    }


/// Decompresses the compressed image file.
/// 
/// # Arguments
/// 
/// * 'filename' - an optional filename of type Option<&String>
pub fn decompress(filename: Option<&str>) {
    let words = match filename {
        Some(file) => read_image_data(Some(file)).expect("Failed to read image data"),
        None => read_image_data(None).expect("Failed to read image data from stdin"),
    };
    let cvc_blocks = blocks_bwc::bwc_to_block(words.clone());
    let cvc_pixels = cvc_block::blocks_to_cvc(cvc_blocks);
    let rgbf64_pixels = rgb_cvc::component_video_to_rgbf64(cvc_pixels);
    let rgb_pixels = rgb_cvc::rgbf64_to_rgb(rgbf64_pixels);
    let image = RgbImage {
        width: words.width as u32,
        height: words.height as u32,
        pixels: rgb_pixels,
        denominator: 255,
    };

    image.write(None).expect("Failed to write image to file");

}
/// Round trips the image file through compression and decompression.
/// 
/// # Arguments
/// 
/// * 'filename' - an optional filename of type Option<&String>
pub fn round_trip(filename: Option<&str>){
    let filename = filename.unwrap();
    let mut image = RgbImage::read(Some(filename)).unwrap();
    image_trim(&mut image);

    // COMPRESSION MODULE
    let mut rgb_pixels = image.pixels;
    let mut rgbf64_pixels = rgb_cvc::rgb_to_rgbf64(rgb_pixels.clone());
    let mut cvc_pixels = rgb_cvc::rgbf64_to_component_video(rgbf64_pixels.clone());
    let mut cvc_blocks = cvc_block::cvc_to_blocks(cvc_pixels.clone(), image.width as usize, image.height as usize);
    let bwc_words = blocks_bwc::block_to_bwc(cvc_blocks); 
    
    // DECOMPRESSION MODULE
    cvc_blocks = blocks_bwc::bwc_to_block(bwc_words);
    cvc_pixels = cvc_block::blocks_to_cvc(cvc_blocks);
    rgbf64_pixels = rgb_cvc::component_video_to_rgbf64(cvc_pixels);
    rgb_pixels = rgb_cvc::rgbf64_to_rgb(rgbf64_pixels);
    image = RgbImage {
        width: image.width,
        height: image.height,
        pixels: rgb_pixels,
        denominator: image.denominator,
    };
    image.write(Some("output.ppm")).expect("Failed to write image to file");


}
