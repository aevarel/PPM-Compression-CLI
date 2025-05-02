use csc411_image::Rgb;
use std::{fs::File, io::{self, BufRead, BufReader}};
use regex::Regex;
use crate::blocks_bwc::WordList;

/// Given a reference to an image vector 'image' of type RgbImage;
/// trims the image to make sure it is even in both dimensions
/// by removing the last row or column or both
/// returns nothing.
/// 
/// # Arguments
/// 
/// * 'image' - a mutable reference to an image vector of type RgbImage
pub fn image_trim(image: &mut csc411_image::RgbImage) {
    // check if the image is even in both dimensions
    // if not, trim the image by one row or column or both
    let mut trimmed_pixels: Vec<Rgb> = Vec::new();
    let mut trimmed_width: u32 = image.width;
    let mut trimmed_height: u32 = image.height;
    if image.height % 2 != 0 {
        trimmed_height -= 1;
    }
    if image.width % 2 != 0 {
        trimmed_width -= 1;
    }
    for row in 0..trimmed_height{
        for col in 0..trimmed_width{
            trimmed_pixels.push(image.pixels[(row*trimmed_width + col) as usize].clone());
        }
    }
    image.width = trimmed_width;
    image.height = trimmed_height;
    image.pixels = trimmed_pixels;
}

/// Given an optional filename 'filename' of type Option<&str>;
/// reads the image data from the file or stdin, expecting the header format "Compressed image format 2",
/// returns a Result of WordList or io::Error.
/// 
/// # Arguments
/// 
/// * 'filename' - an optional filename of type Option<&str>
/// 
/// # Returns
/// 
/// * a Result of WordList or io::Error
pub fn read_image_data(filename: Option<&str>) -> Result<WordList, io::Error> {
    let reader: Box<dyn BufRead> = match filename {
        Some(file) => Box::new(BufReader::new(File::open(file)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };    
    let mut lines = reader.lines();
    // Read the first line
    let first_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing header"))??;    
    // Define the regex pattern to match the header format
    let re = Regex::new(r"Compressed image format 2").unwrap();
    // Match the line against the regex pattern
    if !re.is_match(&first_line) {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid header format"));
    }
    // Read the width and height from the next line
    let dimensions_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing dimensions"))??;
    // Split the dimensions line into width and height
    let dimensions: Vec<&str> = dimensions_line.trim().split_whitespace().collect();
    if dimensions.len() != 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid dimensions format"));
    }
    let width = dimensions[0].parse::<usize>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid width"))?;
    let height = dimensions[1].parse::<usize>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid height"))?;
    // Collect the bwc words
    let mut words = Vec::new();
    for line in lines {
        let line = line?;
        let word = line.trim().parse::<u32>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid bwc word"))?;
        words.push(word);
    }
    Ok(WordList { words, width, height })
}

#[cfg(test)]
mod tests {
    use super::*;
        
    /// Unit test for the image_trim function
    #[test]
    fn test_image_trim() {
        // Create a sample RgbImage with uneven dimensions
        let mut image = csc411_image::RgbImage {
            width: 5,
            height: 4,
            pixels: vec![
                csc411_image::Rgb { red: 255, green: 0, blue: 0 },
                Rgb { red: 0, green: 255, blue: 0 },
                Rgb { red: 0, green: 0, blue: 255 },
                Rgb { red: 255, green: 255, blue: 0 },
                Rgb { red: 255, green: 0, blue: 255 },
                Rgb { red: 0, green: 255, blue: 255 },
                Rgb { red: 128, green: 128, blue: 128 },
                Rgb { red: 255, green: 255, blue: 255 },
                Rgb { red: 0, green: 0, blue: 0 },
                Rgb { red: 128, green: 0, blue: 128 },
                Rgb { red: 0, green: 128, blue: 128 },
                Rgb { red: 128, green: 128, blue: 0 },
                Rgb { red: 255, green: 0, blue: 0 },
                Rgb { red: 0, green: 255, blue: 0 },
                Rgb { red: 0, green: 0, blue: 255 },
                Rgb { red: 255, green: 255, blue: 0 },
                Rgb { red: 255, green: 0, blue: 255 },
                Rgb { red: 0, green: 255, blue: 255 },
                Rgb { red: 128, green: 128, blue: 128 },
                Rgb { red: 255, green: 255, blue: 255 },
            ],
            denominator: 255,
        };
        // Call the image_trim function
        image_trim(&mut image);
        // Check if the dimensions are trimmed to even numbers
        assert_eq!(image.width % 2, 0);
        assert_eq!(image.height % 2, 0);
        // Check if the number of pixels is properly trimmed
        assert_eq!(image.width * image.height, 16);
    }

}


