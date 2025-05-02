pub mod bitpack;
pub use crate::bitpack::fitss;
pub use crate::bitpack::fitsu;
pub use crate::bitpack::gets;
pub use crate::bitpack::getu;
pub use crate::bitpack::news;
pub use crate::bitpack::newu;

#[cfg(test)]
mod tests {
    use crate::bitpack::*;

    #[test]
    fn test_fitss() {
        // Test cases for fitss function
        assert_eq!(fitss(5, 3), false); // Value does not fit into 3 signed bits
        assert_eq!(fitss(5, 4), true);  // Value fits into 4 signed bits
    }

    #[test]
    fn test_fitsu() {
        // Test cases for fitsu function
        assert_eq!(fitsu(5, 3), true);  // Value fits into 3 unsigned bits
        assert_eq!(fitsu(8, 3), false); // Value does not fit into 3 unsigned bits
    }

    #[test]
    fn test_gets() {
        // Test cases for gets function
        assert_eq!(gets(0x3f4, 6, 2), Some(-3));  // Extracting signed value from word
        // Add more test cases as needed
    }

    #[test]
    fn test_getu() {
        // Test cases for getu function
        assert_eq!(getu(0x3f4, 6, 2), Some(61));  // Extracting unsigned value from word
        // Add more test cases as needed
    }

    #[test]
    fn test_newu() {
        assert_eq!(newu(0b1011_1111, 2, 4, 2), Some(0b1010_1111));
    }

    #[test]
    fn test_news() {
        // Test cases for news function
        assert_eq!(news(0b1011_1111, 2, 4, -2), Some(0b1010_1111)); // Updating signed value in word
        // Add more test cases as needed
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
