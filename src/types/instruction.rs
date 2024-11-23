use crate::ops::write::write_memory;

#[derive(Clone)]
pub struct Instruction {
    pub address: *mut u8,
    pub bytes: Vec<u8>,
    pub size: usize,
}

impl Instruction {
    pub fn new(address: *mut u8, bytes: Vec<u8>) -> Self {
        let size = bytes.len();
        Instruction {
            address,
            bytes,
            size,
        }
    }

    /// Restores the original bytes at the specified memory address.
    ///
    /// This function iterates over the saved bytes in the `Instruction` and writes each byte back to the original memory address,
    /// effectively restoring the memory to its previous state. It is commonly used to undo modifications made to executable code or data.
    ///
    /// # Safety
    /// This function is `unsafe` because it performs raw pointer arithmetic and dereferences raw pointers.
    /// - The caller must ensure that the memory address is valid and writable.
    /// - Writing to an invalid or protected memory region may cause undefined behavior or a crash.
    ///
    /// # Example
    /// ```rust
    /// use verity_memory::ops::write::{nop_instructions, write_memory};
    /// use verity_memory::types::instruction::Instruction;
    ///
    /// unsafe {
    ///     // Example machine code (push rbp; mov rbp, rsp; nop; nop)
    ///     let mut buffer = vec![0x55, 0x48, 0x89, 0xE5, 0x90, 0x90];
    ///     let original_buffer = buffer.clone();
    ///     let buffer_ptr = buffer.as_mut_ptr();
    ///     
    ///     // Replace the first instruction with NOP (0x90)
    ///     let original_instructions = nop_instructions(buffer_ptr, 1);
    ///     
    ///     // Check that the original instruction was captured successfully
    ///     assert!(original_instructions.is_some());
    ///     let instruction = original_instructions.unwrap().first().unwrap().clone();
    ///     
    ///     // Manually restore the first instruction using the `restore` method
    ///     instruction.restore();
    ///
    ///     // Assert that the buffer is now identical to the original buffer
    ///     assert_eq!(buffer, original_buffer, "The buffer was not correctly restored to its original state.");
    /// }
    /// ```
    ///
    /// In this example:
    /// 1. We use `nop_instructions` to replace the first instruction in the buffer with a NOP.
    /// 2. The original instruction is captured in an `Instruction` object.
    /// 3. We call the `restore` method directly on the `Instruction` object to revert the change.
    /// 4. Finally, we assert that the buffer matches its original state, confirming successful restoration.
    pub unsafe fn restore(&self) {
        for (i, &byte) in self.bytes.iter().enumerate() {
            let res = write_memory(self.address.add(i), byte);
            if let Err(_) = res {
                return;
            }
        }
    }
}

pub trait InstructionVecExt {
    fn restore_all(&self);
}

impl InstructionVecExt for Vec<Instruction> {
    /// Restores the original bytes at the specified memory address.
    ///
    /// This function iterates over the saved bytes in the `Instruction` and writes each byte back to the original memory address,
    /// effectively restoring the memory to its previous state. It is commonly used to undo modifications made to executable code or data.
    ///
    /// # Safety
    /// This function is `unsafe` because it performs raw pointer arithmetic and dereferences raw pointers.
    /// - The caller must ensure that the memory address is valid and writable.
    /// - Writing to an invalid or protected memory region may cause undefined behavior or a crash.
    ///
    /// # Example
    /// ```rust
    /// use verity_memory::ops::write::{nop_instructions, write_memory};
    /// use verity_memory::types::instruction::InstructionVecExt;
    ///
    /// unsafe {
    ///     // Example machine code (push rbp; mov rbp, rsp; nop; nop)
    ///     let buffer = vec![0x55, 0x48, 0x89, 0xE5, 0x90, 0x90];
    ///     let original_buffer = buffer.clone();
    ///     let buffer_ptr = buffer.as_ptr() as *mut u8;
    ///     
    ///     // Replace the first two instructions with NOPs (0x90)
    ///     let original_instructions = nop_instructions(buffer_ptr, 2);
    ///     
    ///     // Check that original instructions were captured successfully
    ///     assert!(original_instructions.is_some());
    ///     let instructions = original_instructions.unwrap();
    ///     
    ///     // Restore the original instructions using the `restore_all` method
    ///     instructions.restore_all();
    ///
    ///     //Assert that the buffer is now identical to the original buffer
    ///     assert_eq!(buffer, original_buffer, "The buffer was not correctly restored to its original state.");
    /// }
    /// ```
    ///
    /// In this example:
    /// 1. We use `nop_instructions` to replace the first two instructions in the buffer with NOPs.
    /// 2. The original instructions are captured in a vector of `Instruction`.
    /// 3. Finally, we call `restore_all` to revert the changes, restoring the original machine code.
    fn restore_all(&self) {
        for instruction in self {
            unsafe {
                instruction.restore();
            }
        }
    }
}
