// utils.rs
/// Checks if the floating point numbers are approximately equal, which should hold true as an 
/// invariant for being converted back and forth between rgb and cvc, 
/// as this is compression with approximation
/// 
/// # Arguments
/// 
/// * 'a' - the first floating point number
/// * 'b' - the second floating point number
/// * 'tolerance' - the tolerance for the comparison
///
/// # Returns
/// 
/// * a boolean value indicating if the two floating point numbers are approximately equal
pub fn approx_equal(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() < tolerance
}