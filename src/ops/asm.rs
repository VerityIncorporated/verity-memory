use capstone::arch::x86::X86Insn;
use capstone::arch::BuildsCapstone;
use capstone::{Capstone, Insn};
use dynasmrt::dynasm;
use dynasmrt::DynasmApi;

use crate::macros::match_number::{FloatType, IntegerType, IntegralType};
use crate::types::Instruction;

#[cfg(target_arch = "x86_64")]
use dynasmrt::x64::Assembler;
#[cfg(target_arch = "x86")]
use dynasmrt::x86::Assembler;

pub(crate) fn integer_ret(integer_type: IntegerType) -> Vec<u8> {
    let mut assembler = Assembler::new().expect("Failed to create assembler");

    match integer_type {
        IntegerType::I32(value) => {
            dynasm!(assembler
                ; mov eax, *value
                ; ret
            );
        }
        IntegerType::I64(value) => {
            dynasm!(assembler
                ; mov rax, QWORD *value
                ; ret
            );
        }
    }

    let code = assembler.finalize().expect("Failed to finalize assembler");

    let code_slice = unsafe { std::slice::from_raw_parts(code.as_ptr(), code.len()) };
    code_slice.to_vec()
}

pub(crate) fn float_ret(float_type: FloatType) -> Vec<u8> {
    let mut assembler = Assembler::new().expect("Failed to create assembler");

    match float_type {
        FloatType::F32(value) => {
            dynasm!(assembler
                ; mov eax, DWORD value.to_bits().try_into().unwrap()
                ; movd xmm0, eax
                ; ret
            );
        }
        FloatType::F64(value) => {
            dynasm!(assembler
                ; mov rax, QWORD value.to_bits().try_into().unwrap()
                ; movq xmm0, rax
                ; ret
            );
        }
    }

    let code = assembler.finalize().expect("Failed to finalize assembler");

    let code_slice = unsafe { std::slice::from_raw_parts(code.as_ptr(), code.len()) };
    code_slice.to_vec()
}

pub(crate) fn integral_ret(integral_type: IntegralType) -> Vec<u8> {
    let mut assembler = Assembler::new().expect("Failed to create assembler");

    match integral_type {
        IntegralType::U8(value) => {
            dynasm!(assembler
                ; mov eax, (*value).try_into().unwrap()
                ; ret
            );
        }
        IntegralType::U16(value) => {
            dynasm!(assembler
                ; mov ax, (*value).try_into().unwrap()
                ; ret
            );
        }
        IntegralType::U32(value) => {
            dynasm!(assembler
                ; mov eax, (*value).try_into().unwrap()
                ; ret
            );
        }
        IntegralType::U64(value) => {
            dynasm!(assembler
                ; mov rax, (*value).try_into().unwrap()
                ; ret
            );
        }
    }

    let code = assembler.finalize().expect("Failed to finalize assembler");

    let code_slice = unsafe { std::slice::from_raw_parts(code.as_ptr(), code.len()) };
    code_slice.to_vec()
}

pub(crate) fn get_instruction(memory: *mut u8, length: usize) -> Option<Instruction> {
    let cs = Capstone::new()
        .x86()
        .mode(if cfg!(target_arch = "x86_64") {
            capstone::arch::x86::ArchMode::Mode64
        } else {
            capstone::arch::x86::ArchMode::Mode32
        })
        .build()
        .unwrap();

    if memory.is_null() {
        return None;
    }

    let memory_slice: &[u8] = unsafe { std::slice::from_raw_parts(memory, length) };

    let instructions = cs.disasm_all(memory_slice, 0x0).unwrap();

    instructions.get(0).map(|insn: &Insn| {
        let bytes = insn.bytes().to_vec();
        Instruction::new(memory, bytes)
    })
}

pub(crate) fn _get_function(memory: *mut u8) -> Option<Vec<Instruction>> {

    let cs = Capstone::new()
        .x86()
        .mode(if cfg!(target_arch = "x86_64") {
            capstone::arch::x86::ArchMode::Mode64
        } else {
            capstone::arch::x86::ArchMode::Mode32
        })
        .build()
        .unwrap();

    if memory.is_null() {
        return None;
    }

    let mut instructions = Vec::new();
    let mut current_address = memory as usize;
    let max_instructions = 1000;

    for _ in 0..max_instructions {

        let chunk_size = 16;
        let memory_slice: &[u8] = unsafe { std::slice::from_raw_parts(current_address as *mut u8, chunk_size) };
        let disasm_result = cs.disasm_all(memory_slice, current_address as u64);

        let insns = match disasm_result {
            Ok(insns) => insns,
            Err(_) => break,
        };

        if insns.is_empty() {
            break;
        }

        for insn in insns.iter() {

            let bytes = insn.bytes().to_vec();
            let instruction = Instruction::new(insn.address() as *mut u8, bytes);
            instructions.push(instruction);

            current_address += insn.bytes().len();

            let insn_id = insn.id().0;

            if insn_id == X86Insn::X86_INS_RET as u32
                || insn_id == X86Insn::X86_INS_RETF as u32
                || insn_id == X86Insn::X86_INS_RETFQ as u32
                || insn_id == X86Insn::X86_INS_JMP as u32
                || insn_id == X86Insn::X86_INS_LJMP as u32
            {
                return Some(instructions);
            }
        }
    }

    if !instructions.is_empty() {
        Some(instructions)
    } else {
        None
    }
}