use csc411_image::Rgb;

/// File: rgb_cvc.rs 

/// Given a vector of Rgb 'rgb_pixels';
/// maps each pixel to a tuple of f64 values,
/// returns a vector of tuples of f64 values
/// 
/// # Arguments
/// 
/// * 'rgb_pixels' - a vector of Rgb pixels
/// 
/// # Returns
/// 
/// * a vector of tuples of f64 values
pub fn rgb_to_rgbf64(rgb_pixels: Vec<Rgb>) -> Vec<(f64, f64, f64)> {
    // turns vector into iterator, map each pixel to a tuple of f64 values, then collect the iterator into a vector
    rgb_pixels.iter().map(|rgb: &Rgb| ((rgb.red as f64 / 255.0), (rgb.green as f64 / 255.0), (rgb.blue as f64 / 255.0))).collect()
}

/// Given a vector of tuples of f64 values 'rgbf64_pixels';
/// maps each tuple to a Cvc pixel,
/// returns a vector of Cvc pixels
/// 
/// # Arguments
/// 
/// * 'rgbf64_pixels' - a vector of tuples of f64 values
/// 
/// # Returns
/// 
/// * a vector of tuples of f64, meant to represent y,pb,pr converted values.
pub fn rgbf64_to_component_video(rgbf64_pixels: Vec<(f64, f64, f64)>) -> Vec<(f64, f64, f64)>{
    rgbf64_pixels.iter().map(|f64: &(f64, f64, f64)| (
        (0.299 * f64.0) + (0.587 * f64.1) + (0.114 * f64.2), // y
        (-0.168736 * f64.0) - (0.331264 * f64.1) + (0.5 * f64.2), // pb
        (0.5 * f64.0) - (0.418688 * f64.1) - (0.081312 * f64.2) // pr
    )).collect() 
}


/// Given a vector of Cvc 'component_video';
/// maps each Cvc to a tuple of rgb f64 values,
/// returns a vector of tuples of rgbf64 values
/// 
/// # Arguments
/// 
/// * 'component_video' - a vector of Cvc pixels
/// 
/// # Returns
/// 
/// * a vector of tuples of f64 values
pub fn component_video_to_rgbf64(component_video: Vec<(f64, f64, f64)>) -> Vec<(f64, f64, f64)> {
    component_video.iter().map(|cvc_pix: &(f64, f64, f64)| {
        let r = (cvc_pix.0) + (1.402 * cvc_pix.2);
        let g = (cvc_pix.0) - (0.344136 * cvc_pix.1) - (0.714136 * cvc_pix.2);
        let b = (cvc_pix.0) + (1.772 * cvc_pix.1);
        (r, g, b)
    }).collect()
}

/// Given a vector of tuples of f64 values 'f64_pixels';
/// maps each tuple to an Rgb pixel,
/// returns a vector of Rgb pixels
/// 
/// # Arguments
/// 
/// * 'f64_pixels' - a vector of tuples of f64 values
/// 
/// # Returns
/// 
/// * a vector of Rgb pixels
pub fn rgbf64_to_rgb(f64_pixels: Vec<(f64, f64, f64)>) -> Vec<Rgb> {
    f64_pixels.iter().map(|f64: &(f64, f64, f64)| Rgb { 
        red: ((f64.0 * 255.0).round().clamp(0.0, 255.0)) as u16, 
        green: ((f64.1 * 255.0).round().clamp(0.0, 255.0)) as u16, 
        blue: ((f64.2 * 255.0).round().clamp(0.0, 255.0)) as u16 
    }).collect()
}

#[cfg(test)]
mod tests{
    use crate::image::image_trim;
    use super::*;
    use csc411_image::{RgbImage, Read};
    
    #[test]
    pub fn test_round_rgb_cvc(){
        // Create a sample RgbImage
        let mut image = RgbImage::read(Some("src/test_files/black.ppm")).unwrap();
        
        image_trim(&mut image);
        let rgb_pixels = image.pixels;
    
        // Convert Rgb to Cvc
        let rgbf64_pixels = rgb_to_rgbf64(rgb_pixels.clone());
        let cvc_pixels = rgbf64_to_component_video(rgbf64_pixels.clone());
        // Convert Cvc to Rgb
        let rgbf64_pixels2: Vec<(f64, f64, f64)> = component_video_to_rgbf64(cvc_pixels.clone());
        let rgb_pixels2 = rgbf64_to_rgb(rgbf64_pixels2.clone());

        // Check if the two rgb vectors are equal after being converted back and forth
        assert_eq!(rgb_pixels.len(), rgb_pixels2.len());
        for i in 0..rgb_pixels.len() {
            assert_eq!(rgb_pixels[i].red, rgb_pixels2[i].red);
            assert_eq!(rgb_pixels[i].green, rgb_pixels2[i].green);
            assert_eq!(rgb_pixels[i].blue, rgb_pixels2[i].blue);
        }   
    }
}