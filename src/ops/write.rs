use winapi::um::winnt::PAGE_EXECUTE_READWRITE;
use winapi::{shared::minwindef::LPVOID, um::memoryapi::VirtualProtect};

#[cfg(feature = "advanced-write")]
use crate::macros::match_number::{FloatType, IntegerType, IntegralType, NumberType};
#[cfg(feature = "advanced-write")]
use crate::types::Instruction;
use crate::{errors::WriteMemoryError, utils};
#[cfg(feature = "advanced-write")]
use crate::match_number;

#[cfg(feature = "advanced-write")]
use super::asm::{float_ret, get_instruction, integer_ret, integral_ret};

/// Writes a value of type `T` to the specified memory location.
///
/// # Safety
/// This function is unsafe because it directly manipulates raw pointers, which can cause undefined behavior
/// if the pointer is invalid or points to memory that is not writable.
///
/// # Parameters
/// - `dest_ptr`: A mutable pointer to the destination memory where the value will be written.
/// - `value`: The value to write at the destination memory.
///
/// # Returns
/// - `Ok(())` if the value was successfully written to memory.
/// - `Err(WriteMemoryError)` if an error occurred, such as a null pointer or invalid alignment.
///
/// # Errors
/// - `WriteMemoryError::NullPointer` if `dest_ptr` is null.
/// - `WriteMemoryError::InvalidAlignment` if `dest_ptr` is not correctly aligned.
/// - `WriteMemoryError::FailedToChangeProtection` if memory protection could not be modified.
/// - `WriteMemoryError::FailedToRestoreProtection` if memory protection could not be restored.
///
/// # Example
/// ```rust
/// use verity_memory::ops::write;
/// unsafe {
///     let mut value: i32 = 42;
///     let result = write::write_memory(&mut value as *mut i32, 100);
///     assert!(result.is_ok());
///     assert_eq!(value, 100);
/// }
/// ```
pub unsafe fn write_memory<T: Copy>(dest_ptr: *mut T, value: T) -> Result<(), WriteMemoryError> {
    if dest_ptr.is_null() {
        return Err(WriteMemoryError::NullPointer);
    }

    if !utils::check_alignment(dest_ptr) {
        return Err(WriteMemoryError::InvalidAlignment);
    }

    let mut old_protect = 0;
    let size = std::mem::size_of::<T>();

    let res = VirtualProtect(
        dest_ptr as LPVOID,
        size,
        PAGE_EXECUTE_READWRITE,
        &mut old_protect,
    );
    if res == 0 {
        return Err(WriteMemoryError::FailedToChangeProtection);
    }

    *dest_ptr = value;

    let res_restore = VirtualProtect(dest_ptr as LPVOID, size, old_protect, &mut old_protect);
    if res_restore == 0 {
        return Err(WriteMemoryError::FailedToRestoreProtection);
    }

    Ok(())
}

/// Replaces a specified number of instructions at a memory location with NOPs (0x90).
///
/// # Safety
/// This function is unsafe because it directly modifies memory, which can corrupt the process
/// if the memory is not writable or if the replaced instructions are critical.
///
/// # Parameters
/// - `dest_ptr`: A mutable pointer to the memory location where instructions will be replaced.
/// - `num_instructions`: The number of instructions to replace with NOPs.
///
/// # Returns
/// - `Some(Vec<Instruction>)` containing the original instructions if successful.
/// - `None` if it failed to read instructions or write memory.
///
/// # Example
/// ```rust
/// use verity_memory::ops::write;
/// unsafe {
///     let buffer = vec![0x55, 0x48, 0x89, 0xE5]; // Some sample machine code (push rbp; mov rbp, rsp)
///     let original_instructions = write::nop_instructions(buffer.as_ptr() as *mut u8, 2);
///     assert!(original_instructions.is_some());
/// }
/// ```
#[cfg(feature = "advanced-write")]
pub unsafe fn nop_instructions(dest_ptr: *mut u8, num_instructions: usize) -> Option<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let mut current_ptr = dest_ptr;

    if num_instructions <= 0 {
        panic!("You must nop at least one instruction...");
    }

    for _ in 0..num_instructions {
        if let Some(instr) = get_instruction(current_ptr, 16) {
            instructions.push(instr.clone());
            current_ptr = current_ptr.add(instr.size);
        } else {
            eprintln!("Failed to get instruction at memory address: {:?}", current_ptr);
            return None;
        }
    }

    let total_size: usize = instructions.iter().map(|instr| instr.size).sum();

    let nops = vec![0x90; total_size];
    let mut written_size = 0;
    for i in 0..num_instructions {
        let instruction = &instructions[i];
        for j in 0..instruction.size {
            let res = write_memory(dest_ptr.add(written_size + j), nops[written_size + j]);
            if let Err(e) = res {
                eprintln!("Failed to write memory at offset {}: {:?}", written_size + j, e);
                return None;
            }
        }
        written_size += instruction.size;
    }

    Some(instructions)
}

/// Replaces the return value of a function with a specified value or inserts a `RET` instruction.
///
/// # Safety
/// This function is unsafe because it directly modifies memory, which can cause undefined behavior
/// if the memory is not writable or if the return value type is not correctly handled.
///
/// # Parameters
/// - `dest_ptr`: A mutable pointer to the function's first instruction.
/// - `return_value`: An optional value to return. If `None`, a `RET` instruction is written instead.
///
/// # Returns
/// - `Some(Instruction)` containing the original instruction if successful.
/// - `None` if an error occurred during instruction writing.
///
/// # Example
/// ```rust
/// use verity_memory::ops::write;
/// unsafe {
///     let buffer = vec![0x55, 0x48, 0x89, 0xE5]; // Example machine code
///     let result = write::replace_return_value::<i32>(buffer.as_ptr() as *mut u8, Some(123));
///     assert!(result.is_some());
/// }
/// ```
#[cfg(feature = "advanced-write")]
pub unsafe fn replace_return_value<T: Copy + 'static>(
    dest_ptr: *mut u8,
    return_value: Option<T>,
) -> Option<Instruction> {
    let original_instruction = get_instruction(dest_ptr, 16)?;

    let value = match return_value {
        Some(val) => val,
        None => {
            if let Err(e) = write_memory(dest_ptr, 0xC3) {
                eprintln!("Failed to write RET instruction: {:?}", e);
                return None;
            }
            return Some(original_instruction);
        }
    };

    let result = match_number!(value);
    if result.is_none() {
        return None;
    }

    let number_type = result.unwrap();
    let instruction_bytes = match number_type {
        NumberType::Float(float_type) => float_ret(float_type),
        NumberType::Integer(integer_type) => integer_ret(integer_type),
        NumberType::Integral(integral_type) => integral_ret(integral_type),
        NumberType::Unknown => {
            return None;
        }
    };

    let mut current_ptr = dest_ptr;
    for instruction_byte in instruction_bytes {
        if write_memory(current_ptr, instruction_byte).is_err() {
            return None;
        }
        current_ptr = current_ptr.add(1);
    }

    Some(original_instruction)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    fn mock_dest_ptr<T: Copy>(value: T) -> *mut T {
        let mut boxed_value = Box::new(value);
        let ptr = &mut *boxed_value as *mut T;
        std::mem::forget(boxed_value);
        ptr
    }

    #[test]
    fn test_write_memory_success() {
        let value: u32 = 42;
        let dest_ptr = mock_dest_ptr(value);

        let result = unsafe { write_memory(dest_ptr, 100_u32) };
        assert!(result.is_ok());
        assert_eq!(unsafe { *dest_ptr }, 100_u32);
    }

    #[test]
    fn test_write_memory_null_pointer() {
        let dest_ptr: *mut u32 = ptr::null_mut();
        
        let result = unsafe { write_memory(dest_ptr, 100_u32) };
        assert!(matches!(result, Err(WriteMemoryError::NullPointer)));
    }
    
    #[test]
    #[cfg(feature = "advanced-write")]
    fn test_nop_instructions_success() {
        let data: Vec<u8> = vec![0x55, 0x48, 0x8B, 0xEC, 0x90];
        let dest_ptr = data.as_ptr() as *mut u8;

        unsafe {
            if let Some(instructions) = nop_instructions(dest_ptr, 2) {
                assert_eq!(instructions.len(), 2);
            } else {
                panic!("Failed to retrieve instructions");
            }
        }
    }

    #[test]
    #[cfg(feature = "advanced-write")]
    fn test_nop_instructions_failure() {
        let dest_ptr: *mut u8 = ptr::null_mut();

        unsafe {
            let instructions = nop_instructions(dest_ptr, 1);
            assert!(instructions.is_none());
        }
    }

    #[test]
    #[cfg(feature = "advanced-write")]
    fn test_replace_return_value_integer() {
        let data: Vec<u8> = vec![0x55, 0x48, 0x8B, 0xEC];
        let dest_ptr = data.as_ptr() as *mut u8;

        unsafe {
            let result = replace_return_value(dest_ptr, Some(123_u32));
            assert!(result.is_some());
        }
    }

    #[test]
    #[cfg(feature = "advanced-write")]
    fn test_replace_return_value_float() {
        let data: Vec<u8> = vec![0x55, 0x48, 0x8B, 0xEC];
        let dest_ptr = data.as_ptr() as *mut u8;

        unsafe {
            let result = replace_return_value(dest_ptr, Some(123.45_f32));
            assert!(result.is_some());
        }
    }

    #[test]
    #[cfg(feature = "advanced-write")]
    fn test_replace_return_value_none() {
        let data: Vec<u8> = vec![0x55, 0x48, 0x8B, 0xEC];
        let dest_ptr = data.as_ptr() as *mut u8;

        unsafe {
            let result = replace_return_value::<i32>(dest_ptr, None);
            assert!(result.is_some());
        }
    }
}