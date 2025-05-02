use csc411_arith::{chroma_of_index, index_of_chroma};
use crate::cvc_block::CvcBlock;
use bitpack::bitpack;


/// A struct that contains a list of words, width, and height
/// 
/// # Attributes
/// 
/// * 'words' - a vector of words
/// * 'width' - the width of the image
/// * 'height' - the height of the image
#[derive(Clone)]
pub struct WordList {
    // list of words
    pub words: Vec<u32>,
    // inherits the width and height of the image
    pub width: usize,
    pub height: usize
}

/// Given a 2x2 block of ComponentVideoCode 'block';
/// maps each block to a BitpackedWord,
/// returns a BitpackedWord list
/// 
/// # Arguments
/// 
/// * 'blocks' - a ComponentBlocks struct
/// 
/// # Returns
/// 
///  * a WordList struct
pub fn block_to_bwc(blocks: CvcBlock) -> WordList{
    let mut word_list = WordList {
        words: Vec::new(),
        width: blocks.width,
        height: blocks.height 
    };
    for block in blocks.pixels.iter() {
        // initialize word 
         // unsigned 9-bit scaled integer // multiply by 511 and round to nearest integer
        let a = (((block.3.0 + block.2.0 + block.1.0 + block.0.0) / 4.0) * 511.0).round() as u64;
        // signed 5-bit scaled integer // signed-integer set {−15, −14, . . . , −1, 0, 1, 2, . . . , 15} 
        let b = (((block.3.0 + block.2.0 - block.1.0 - block.0.0) / 4.0).clamp(-0.3,0.3) * (50.0)).clamp(-15.0,15.0) as i64;
        let c = (((block.3.0 - block.2.0 + block.1.0 - block.0.0) / 4.0).clamp(-0.3,0.3) * (50.0)).clamp(-15.0,15.0) as i64;
        let d = (((block.3.0 - block.2.0 - block.1.0 + block.0.0) / 4.0).clamp(-0.3,0.3) * (50.0)).clamp(-15.0,15.0) as i64;
        // average of the chroma values of the four pixels
        
        let pb = ((block.3.1 + block.2.1 + block.1.1 + block.0.1) / 4.0).clamp(-0.35, 0.35) as f64;
        let pr = ((block.3.2 + block.2.2 + block.1.2 + block.0.2) / 4.0).clamp(-0.35, 0.35) as f64;

        let bwc = pack_word(a, b, c, d, pb, pr) as u32;
        word_list.words.push(bwc);
    }
    word_list
}

/// Given a WordList 'words', which is a list of packed bitword codes for 2x2 blocks of ComponentVideoCode;
/// maps each word to a 2x2 block of ComponentVideoCode,
/// returns a CvcBlock struct
/// 
/// # Arguments
/// 
/// * 'words' - a WordList struct
/// 
/// # Returns
/// 
/// * a CvcBlock struct
pub fn bwc_to_block(words: WordList) -> CvcBlock {
    let mut blocks = CvcBlock {
        width: words.width,
        height: words.height,
        pixels: Vec::new()
    };
    for word in words.words.iter() {
        let (a, b, c, d, pb, pr) = unpack_word(*word as u64);

        let a = a as f64 / 511.0;
        let b = b as f64 / 50.0;
        let c = c as f64 / 50.0;
        let d = d as f64 / 50.0;

        let y1 = a - b - c + d;
        let y2 = a - b + c - d;
        let y3 = a  + b - c - d;
        let y4 = a + b + c + d;
        let pb = pb;
        let pr = pr;

        blocks.pixels.push(((y1 as f64, pb, pr), (y2 as f64, pb, pr), (y3 as f64, pb, pr), (y4 as f64, pb, pr)));
    }
    blocks
}

/// Given a 32-bit word 'word';
/// unpacks the word into its components,
/// returns a tuple of the components
/// 
/// # Arguments
/// 
/// * 'word' - a 32-bit word
/// 
/// # Returns
/// 
/// * a tuple of the components
/// 
pub fn unpack_word(word: u64) -> (u64, i64, i64, i64, f64, f64) {
    let a = bitpack::getu(word, 9, 23).unwrap();
    let b = bitpack::gets(word, 5, 18).unwrap();
    let c = bitpack::gets(word, 5, 13).unwrap();
    let d = bitpack::gets(word, 5, 8).unwrap();
    let pb = bitpack::getu(word, 4, 4).unwrap();
    let pr: u64 = bitpack::getu(word, 4, 0).unwrap();
    (a, b, c, d, chroma_of_index(pb as usize) as f64, chroma_of_index(pr as usize) as f64)
}

/// Given a 9-bit unsigned integer 'a', a 5-bit signed integer 'b', a 5-bit signed integer 'c', a 5-bit signed integer 'd', a 4-bit unsigned integer 'pb', and a 4-bit unsigned integer 'pr';
/// packs the components into a 32-bit word,
/// returns the packed word
/// 
/// # Arguments
/// 
/// * 'a' - a 9-bit unsigned integer
/// * 'b' - a 5-bit signed integer
/// * 'c' - a 5-bit signed integer
/// * 'd' - a 5-bit signed integer
/// * 'pb' - a 4-bit unsigned integer
/// * 'pr' - a 4-bit unsigned integer
/// 
/// # Returns
/// 
/// * a 32-bit word
pub fn pack_word(a: u64, b: i64, c: i64, d: i64, pb: f64, pr: f64) -> u32 {
    let mut word: u64 = 0;
    word = bitpack::newu(word, 9, 23, a).unwrap();
    word = bitpack::news(word, 5, 18, b).unwrap();
    word = bitpack::news(word, 5, 13, c).unwrap();
    word = bitpack::news(word, 5, 8, d).unwrap();
    word = bitpack::newu(word, 4, 4, index_of_chroma(pb as f32) as u64).unwrap();
    word = bitpack::newu(word, 4, 0, index_of_chroma(pr as f32) as u64).unwrap();
    word as u32
}


#[cfg(test)]
mod tests {
    use super::*;
    use csc411_image::{RgbImage, Read};
    use crate::image::image_trim;
    use crate::rgb_cvc::{rgb_to_rgbf64, rgbf64_to_component_video};
    use crate::cvc_block::cvc_to_blocks;
    use crate::utils::approx_equal;


    /// Helper function to check if two f64 values are approximately equal by a certain tolerance
    ///
    /// # Arguments
    /// 
    /// * 'a' - a f64 value
    /// * 'b' - a f64 value
    /// * 'tolerance' - a f64 value
    /// 
    /// # Returns
    /// 
    /// * a boolean values
    fn approx_equal_with_tolerance(a: f64, b: f64, tolerance: f64) -> bool {
        let result = (a - b).abs() <= tolerance;
        if !result {
            println!("Assertion failed: a = {}, b = {}, tolerance = {}", a, b, tolerance);
        }
        result
        }
    
    /// Unit test for the pack_word and unpack_word functions
    #[test]
    pub fn test_pack_word() {
        // pack a sample word and unpack it
        let a = 255;
        let b = 15;
        let c = -14;
        let d = 2;
        let pb = 0.35;
        let pr = -0.26;
        let word = pack_word(a, b, c, d, pb, pr);
        println!("word: {}", word);
        let (a2, b2, c2, d2, pb2, pr2) = unpack_word(word as u64);
        println!("a: {}, b: {}, c: {}, d: {}, pb: {}, pr: {}", a2, b2, c2, d2, pb2, pr2);
        assert_eq!(a, a2);
        assert_eq!(b, b2);
        assert_eq!(c, c2);
        assert_eq!(d, d2);
        assert!(approx_equal(pb as f64, pb2 as f64, 0.1));
        assert!(approx_equal(pr as f64, pr2 as f64, 0.1));    
    }

    /// Unit test for the block_to_bwc and bwc_to_block functions    
    #[test]
    pub fn test_round_blocks_bwc(){
        // Create a sample RgbImage
        let mut image = RgbImage::read(Some("src/test_files/black.ppm")).unwrap();
        image_trim(&mut image);
        let rgb_pixels = image.pixels;

        // Convert Rgb to Cvc
        let rgbf64_pixels = rgb_to_rgbf64(rgb_pixels.clone());
        let cvc_pixels = rgbf64_to_component_video(rgbf64_pixels.clone());
        // create cvc_blocks
        let cvc_blocks = cvc_to_blocks(cvc_pixels.clone(), image.width as usize, image.height as usize);
        // create bwc_words
        let bwc_words = block_to_bwc(cvc_blocks.clone());
        // convert bwc_words back to cvc_blocks
        let cvc_blocks2 = bwc_to_block(bwc_words.clone());

        // Check if the two cvc_blocks are equal after being converted back and forth
        assert_eq!(cvc_blocks.width, cvc_blocks2.width);
        assert_eq!(cvc_blocks.height, cvc_blocks2.height);
        assert_eq!(cvc_blocks.pixels.len(), cvc_blocks2.pixels.len());

        // tests to see that they are approximately equal
        for i in 0..cvc_blocks.pixels.len() {
            for _j in 0..4 {
                // print all of the values of each group of 4 pixels
                assert!(approx_equal_with_tolerance(cvc_blocks.pixels[i].0 .0, cvc_blocks2.pixels[i].0 .0, 0.30));
                assert!(approx_equal_with_tolerance(cvc_blocks.pixels[i].0 .1, cvc_blocks2.pixels[i].0 .1, 0.30));
                assert!(approx_equal_with_tolerance(cvc_blocks.pixels[i].0 .2, cvc_blocks2.pixels[i].0 .2, 0.30));            }
        }
    }
}