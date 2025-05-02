
// Module for handling 2x2 blocks of pixels in component video code format

/// Struct to represent a 2x2 block of pixels in component video code format
/// 
/// # Fields
/// 
/// * 'width' - the width of the image
/// * 'height' - the height of the image
/// * 'pixels' - a vector of 2x2 blocks of pixels, each with Y, Pb, Pr values
#[derive(Clone)]
pub struct CvcBlock {
    pub width: usize,
    pub height: usize,
    // 2x2 block of pixels, each with Y, Pb, Pr values
    // Notated as (Y1, Pb1, Pr1, Y2, Pb2, Pr2, Y3, Pb3, Pr3, Y4, Pb4, Pr4)
    pub pixels: Vec<((f64,f64,f64),(f64,f64,f64),(f64,f64,f64),(f64,f64,f64))>,
}


/// Given a vector of component video code pixel data tuples 'cvc_pixel' and the width and height of the image,
/// generates a collection of these pixels in 2x2 blocks, and returns a CvcBlock struct, with width, height, 
/// and the pixels of the original image.
/// 
/// # Arguments
/// 
/// * 'cvc_pixel' - a vector of tuples of component video code pixel data
/// * 'width' - the width of the image
/// * 'height' - the height of the image
/// 
/// # Returns
/// 
/// * a CvcBlock struct, with width, height, and the pixels of the passed image data
pub fn cvc_to_blocks(cvc_pixel: Vec<(f64,f64,f64)>, width: usize, height: usize) -> CvcBlock {
    let mut blocks: CvcBlock = CvcBlock {
        width: width,
        height: height,
        pixels: Vec::new(),
    };
    // iterate over each pixel in blocks of 2x2
    for i in (0..height).step_by(2) {
        for j in (0..width).step_by(2) {
            let y1 = cvc_pixel[i * width + j]; // top left
            let y2 = cvc_pixel[i * width + j + 1]; // top right
            let y3 = cvc_pixel[(i + 1) * width + j]; // bottom left
            let y4 = cvc_pixel[(i + 1) * width + j + 1]; // bottom right 
            
            blocks.pixels.push((y1, y2, y3, y4));
        }
    }
    blocks
}

/// Given a CvcBlock struct 'blocks', which is the original image's component video code data in 2x2 blocks,
/// returns a vector of component video code pixel data tuples
/// 
/// # Arguments
/// 
/// * 'blocks' - a CvcBlock struct, which is the original image's component video code data in 2x2 blocks
/// 
/// # Returns
/// 
/// * a vector of tuples of component video code pixel data
pub fn blocks_to_cvc(blocks: CvcBlock) -> Vec<(f64, f64, f64)> {
    let mut cvc_pixel = vec![(0.0, 0.0, 0.0); blocks.width * blocks.height];
    for (block_index, block) in blocks.pixels.iter().enumerate() {
        let block_row = block_index / (blocks.width / 2);
        let block_col = block_index % (blocks.width / 2);
        let i = block_row * 2;
        let j = block_col * 2;
        cvc_pixel[i * blocks.width + j] = block.0; // top left
        
        if j + 1 < blocks.width {
            cvc_pixel[i * blocks.width + j + 1] = block.1; // top right
        }
        if i + 1 < blocks.height {
            cvc_pixel[(i + 1) * blocks.width + j] = block.2; // bottom left
        }
        if i + 1 < blocks.height && j + 1 < blocks.width {
            cvc_pixel[(i + 1) * blocks.width + j + 1] = block.3; // bottom right
        }
    }
    cvc_pixel
}


#[cfg(test)]
mod tests {
    use super::*;
    use csc411_image::{RgbImage, Read};
    use crate::image::image_trim;
    use crate::rgb_cvc::{rgb_to_rgbf64, rgbf64_to_component_video, component_video_to_rgbf64, rgbf64_to_rgb};
    use crate::utils::approx_equal;


    /// Unit test for round-tripping a 2x2 block of pixels in component video code format
    #[test]
    pub fn test_round_cvc_blocks() {
        // Create a sample RgbImage
        let mut image = RgbImage::read(Some("src/test_files/black.ppm")).unwrap();
        image_trim(&mut image);
        let rgb_pixels = image.pixels;
    
        // Convert Rgb to Cvc
        let rgbf64_pixels = rgb_to_rgbf64(rgb_pixels.clone());
        let cvc_pixels = rgbf64_to_component_video(rgbf64_pixels.clone());
        // create cvc_blocks
        let cvc_blocks = cvc_to_blocks(cvc_pixels.clone(), image.width as usize, image.height as usize);
        
        // Convert Cvc to Rgb
        let cvc_pixels2 = blocks_to_cvc(cvc_blocks.clone());
        let rgbf64_pixels2 = component_video_to_rgbf64(cvc_pixels2.clone());
        let rgb_pixels2 = rgbf64_to_rgb(rgbf64_pixels2.clone());

        // Define a tolerance for floating-point comparisons
        let tolerance = 1e-6;

        // Check if the two cvc vectors are equal after being converted back and forth
        assert_eq!(cvc_pixels.len(), cvc_pixels2.len());
        for i in 0..cvc_pixels.len() {
            assert!(approx_equal(cvc_pixels[i].0, cvc_pixels2[i].0, tolerance), "cvc_pixels.0: {}, cvc_pixels2.0: {}", cvc_pixels[i].0, cvc_pixels2[i].0);
            assert!(approx_equal(cvc_pixels[i].1, cvc_pixels2[i].1, tolerance), "cvc_pixels.1: {}, cvc_pixels2.1: {}", cvc_pixels[i].1, cvc_pixels2[i].1);
            assert!(approx_equal(cvc_pixels[i].2, cvc_pixels2[i].2, tolerance), "cvc_pixels.2: {}, cvc_pixels2.2: {}", cvc_pixels[i].2, cvc_pixels2[i].2);
        }
        // Check if the two rgb vectors are equal after being converted back and forth
        assert_eq!(rgb_pixels.len(), rgb_pixels2.len());
        for i in 0..rgb_pixels.len() {
            assert_eq!(rgb_pixels[i].red, rgb_pixels2[i].red);
            assert_eq!(rgb_pixels[i].green, rgb_pixels2[i].green);
            assert_eq!(rgb_pixels[i].blue, rgb_pixels2[i].blue);
        }
    }
}