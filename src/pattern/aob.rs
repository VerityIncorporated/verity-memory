use crate::{
    errors::AobScanError,
    pattern::algorithm::{convert_pattern, kmp_search_all, kmp_search_unique},
};

use super::memory::get_text_section;

/// # Safety
///
/// This function is unsafe because it involves direct manipulation of memory pointers. The caller
/// must ensure that the returned pointer is handled safely.
///
/// # Description
///
/// Scans the text section of the current process's memory for a unique occurrence of a byte pattern
/// specified by the given `pattern` string.
///
/// This function uses the Knuth-Morris-Pratt (KMP) algorithm to efficiently search for the byte pattern.
/// If the pattern is found, it returns a mutable pointer to the first byte of the matched pattern.
///
/// # Parameters
/// - `pattern`: A string representing the byte pattern to search for. This pattern must be formatted as
///   a hexadecimal string with wildcards (e.g., `"48 8B ?? ?? 89 ?? 74 0F"`).
///
/// # Returns
/// - `Ok(*mut u8)`: A mutable pointer to the first byte of the unique matched pattern.
/// - `Err(AobScanError)`: An error if the pattern is not found or is invalid.
///
/// # Errors
/// - `AobScanError::PatternNotFound`: Returned if the pattern is not found in the text section.
/// - `AobScanError::InvalidPattern`: Returned if the pattern string is invalid.
///
/// # Examples
/// ```
/// use verity_memory::pattern::aob;
///
/// unsafe {
///     match aob::scan_unique("48 8B ?? ?? 89 ?? 74 0F") {
///         Ok(ptr) => println!("Pattern found at address: {:?}", ptr),
///         Err(e) => println!("Failed to find pattern: {}", e),
///     }
/// }
/// ```
pub unsafe fn scan_unique(pattern: &str) -> Result<*mut u8, AobScanError> {
    let pattern_bytes = convert_pattern(pattern)?;
    let test_region = get_text_section();

    let index = kmp_search_unique(&test_region.0, &pattern_bytes)?;
    Ok((test_region.1 + index) as *mut u8)
}

/// # Safety
///
/// This function is unsafe because it involves direct manipulation of memory pointers. The caller
/// must ensure that the returned pointers are handled safely.
///
/// # Description
///
/// Scans the text section of the current process's memory for all occurrences of a byte pattern
/// specified by the given `pattern` string.
///
/// This function uses the Knuth-Morris-Pratt (KMP) algorithm to efficiently search for the byte pattern.
/// It returns a vector of mutable pointers to the first byte of each matched pattern.
///
/// # Parameters
/// - `pattern`: A string representing the byte pattern to search for. This pattern must be formatted as
///   a hexadecimal string with wildcards (e.g., `"48 8B ?? ?? 89 ?? 74 0F"`).
///
/// # Returns
/// - `Ok(Vec<*mut u8>)`: A vector of mutable pointers to the first byte of each matched pattern.
/// - `Err(AobScanError)`: An error if the pattern is not found or is invalid.
///
/// # Errors
/// - `AobScanError::PatternNotFound`: Returned if no occurrences of the pattern are found.
/// - `AobScanError::InvalidPattern`: Returned if the pattern string is invalid.
///
/// # Examples
/// ```
/// use verity_memory::pattern::aob;
///
/// unsafe {
///     match aob::scan_all("48 8B ?? ?? 89 ?? 74 0F") {
///         Ok(ptrs) => {
///             for ptr in ptrs {
///                 println!("Pattern found at address: {:?}", ptr);
///             }
///         }
///         Err(e) => println!("Failed to find pattern: {}", e),
///     }
/// }
/// ```
pub unsafe fn scan_all(pattern: &str) -> Result<Vec<*mut u8>, AobScanError> {
    let pattern_bytes = convert_pattern(pattern)?;
    let test_region = get_text_section();

    let indices = kmp_search_all(&test_region.0, &pattern_bytes)?;
    Ok(indices
        .into_iter()
        .map(|index| (test_region.1 + index) as *mut u8)
        .collect())
}