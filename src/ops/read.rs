use std::panic::{catch_unwind, AssertUnwindSafe};

use winapi::{shared::minwindef::LPVOID, um::{memoryapi::VirtualProtect, winnt::PAGE_EXECUTE_READWRITE}};

use crate::{errors::ReadMemoryError, utils};

/// Reads a value from the specified memory address with the specified type.
/// 
/// # Safety
/// This function is `unsafe` because it dereferences a raw pointer, which could lead to undefined behavior if the pointer is invalid.
/// 
/// # Type Parameters
/// - `T`: The type of value to read. It must implement the `Copy` trait.
/// 
/// # Parameters
/// - `address`: A raw pointer to the memory location from which to read.
/// 
/// # Returns
/// - `Ok(T)`: The value read from the specified memory address if successful.
/// - `Err(ReadMemoryError)`: Returns an error if the pointer is null, misaligned, or the read operation fails.
/// 
/// # Errors
/// - `ReadMemoryError::NullPointer`: If the provided pointer is null.
/// - `ReadMemoryError::InvalidAlignment`: If the provided pointer is not correctly aligned for the type `T`.
/// - `ReadMemoryError::FailedToChangeProtection`: If changing the memory protection fails.
/// - `ReadMemoryError::FailedToRestoreProtection`: If restoring the memory protection fails.
/// - `ReadMemoryError::InvalidAccess`: If there is an error during the read operation.
/// 
/// # Example
/// ```
/// use verity_memory::ops::read;
/// let address: *const i32 = 0x12345678 as *const i32;
/// let result = unsafe { read::read_memory(address) };
/// match result {
///     Ok(value) => println!("Value read: {}", value),
///     Err(e) => println!("Failed to read memory: {:?}", e),
/// }
/// ```
pub unsafe fn read_memory<T: Copy>(address: *const T) -> Result<T, ReadMemoryError> {
    if address.is_null() {
        return Err(ReadMemoryError::NullPointer);
    }

    if !utils::check_alignment(address) {
        return Err(ReadMemoryError::InvalidAlignment);
    }

    let mut old_protect = 0;
    let size = std::mem::size_of::<T>();

    let res = VirtualProtect(
        address as LPVOID,
        size,
        PAGE_EXECUTE_READWRITE,
        &mut old_protect,
    );

    if res == 0 {
        return Err(ReadMemoryError::FailedToChangeProtection);
    }

    let result = catch_unwind(AssertUnwindSafe(|| *address))
        .map_err(|_| ReadMemoryError::InvalidAccess);

    let res_restore = VirtualProtect(address as LPVOID, size, old_protect, &mut old_protect);
    if res_restore == 0 {
        return Err(ReadMemoryError::FailedToRestoreProtection);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_memory_valid() {
        let valid_data = 42;
        let ptr: *const i32 = &valid_data;

        let result = unsafe { read_memory(ptr) };
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_read_memory_null_pointer() {
        let null_ptr: *const i32 = std::ptr::null();

        let result = unsafe { read_memory(null_ptr) };
        assert_eq!(result, Err(ReadMemoryError::NullPointer));
    }

    #[test]
    fn test_read_memory_invalid_alignment() {
        let invalid_data = 42i32;
        let ptr: *const i32 = &invalid_data;

        let unaligned_ptr = (ptr as usize + 1) as *const i32;

        let result = unsafe { read_memory(unaligned_ptr) };
        assert_eq!(result, Err(ReadMemoryError::InvalidAlignment));
    }
}