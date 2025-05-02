
/// Returns true iff the signed value `n` fits into `width` signed bits.
///
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    let min = -(1 << (width - 1));
    let max = (1 << (width - 1)) - 1;
    n >= min && n <= max
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
///
/// # Arguments:
/// * `n`: An unsigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    n <= (1 << width) - 1
}

/// Retrieve a signed value from an unsigned `word`,
/// beginning at least significant bit `lsb`
/// and having `width` bits.
///
/// # Arguments
///
/// * `word` - the word from which to extract a value
/// * `width` - the number of bits in the field
/// * `lsb` - the least-significant bit of the field
///
/// # Returns
///
/// a signed value corresponding to the 2s complement representation
/// of the appropriate field of the `word`
/// or `None` if the field is impossible
pub fn gets(word: u64, width: u64, lsb: u64) -> Option<i64> {
    /*Calling bitpack::getu or bitpack::gets with a width w that does not satisfy 0 ≤ w ≤ 64 
    should result in an Err from the Result<T,E>. */
    if lsb + width > 64 {
        return None;
    }
    let mask = (1 << width) - 1;
    let value = (word >> lsb) & mask;
    let sign_extend = if value & (1 << (width - 1)) != 0 {
        !0 << width
    } else {
        0
    };
    Some((value | sign_extend) as i64)
}

/// Retrieve a signed value from an unsigned `word`,
/// beginning at least significant bit `lsb`
/// and having `width` bytes.
///
/// # Arguments
///
/// * `word` - the word from which to extract a value
/// * `width` - the number of bits in the field
/// * `lsb` - the least-significant bit of the field
///
/// # Returns
///
/// a signed value corresponding to the 2s complement representation
/// of the appropriate field of the `word`
///
/// or None
/// if `lsb + width > 64`
///
pub fn getu(word: u64, width: u64, lsb: u64) -> Option<u64> {
    if lsb + width > 64 {
        return None;
    }
    let mask = (1 << width) - 1;
    Some((word >> lsb) & mask)
}

/// Given an unsigned 64-bit `word`, and an unsigned `value`,
/// pack that `value` into `width` bits of the `word` starting at
/// least-significant bit `lsb`, if possible.
///
/// # Arguments
///
/// * `word` - an arbitrary unsigned 64-bit word
/// * `width` - a number of bits describing a field
/// * `lsb` - the-least significant bit of a field
/// * `value` - the unsigned value to store in the field
///
/// # Returns
///
/// an `Option<u64>` which contains the desired value at the appropriate field, if possible
/// If the value does not fit, returns `None`
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if lsb + width > 64 {
        return None;
    }
    let mask = ((1 << width) - 1) << lsb;
    let cleared_word = word & !mask;
    Some(cleared_word | ((value & ((1 << width) - 1)) << lsb))
}

/// Given an unsigned 64-bit `word`, and a signed `value`,
/// pack that `value` into `width` bits of the `word` starting at
/// least-significant bit `lsb`, if possible.
///
/// # Arguments
///
/// * `word` - an arbitrary unsigned 64-bit word
/// * `width` - a number of bits describing a field
/// * `lsb` - the-least significant bit of a field
/// * `value` - the signed value to store in the field
///
/// # Returns
///
/// an `Option<u64>` which contains the desired value at the appropriate field, if possible
/// If the value does not fit, returns `None`
///
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if lsb + width > 64 || !fitss(value, width) {
        return None;
    }
    let mask = ((1 << width) - 1) << lsb;
    let cleared_word = word & !mask;
    Some(cleared_word | (((value as u64) & ((1 << width) - 1)) << lsb))
}
