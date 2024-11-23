use std::ptr;
use std::slice;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::winnt::{
    IMAGE_DOS_HEADER, IMAGE_SECTION_HEADER,
};

#[cfg(target_arch = "x86")]
use winapi::um::winnt::IMAGE_NT_HEADERS32;

#[cfg(target_arch = "x86_64")]
use winapi::um::winnt::IMAGE_NT_HEADERS64;

pub(crate) unsafe fn get_text_section() -> (Vec<u8>, usize) {

    let base_address = GetModuleHandleA(ptr::null());
    if base_address.is_null() {
        panic!("Failed to get module handle");
    }
    let base_address = base_address as usize;

    let dos_header = &*(base_address as *const IMAGE_DOS_HEADER);
    if dos_header.e_magic != 0x5A4D {
        panic!("Invalid DOS header signature");
    }

    let nt_header_ptr = base_address + dos_header.e_lfanew as usize;
    let signature = *(nt_header_ptr as *const u32);
    if signature != 0x4550 {
        panic!("Invalid NT header signature");
    }

    let (number_of_sections, section_header_ptr) = get_nt_headers(nt_header_ptr);

    let mut section = section_header_ptr as *const IMAGE_SECTION_HEADER;
    let mut text_section_ptr: *const IMAGE_SECTION_HEADER = ptr::null();

    for _ in 0..number_of_sections {
        let section_name = (*section).Name;
        if section_name.starts_with(b".text") {
            text_section_ptr = section;
            break;
        }
        section = section.add(1);
    }

    if text_section_ptr.is_null() {
        panic!("Failed to locate .text section");
    }

    let text_section = &*text_section_ptr;
    let text_address = base_address + text_section.VirtualAddress as usize;
    let text_size = text_section.SizeOfRawData as usize;

    let text_slice = slice::from_raw_parts(text_address as *const u8, text_size);

    (text_slice.to_vec(), text_address)
    
}

#[cfg(target_arch = "x86_64")]
unsafe fn get_nt_headers(nt_header_ptr: usize) -> (usize, usize) {
    let nt_headers = &*(nt_header_ptr as *const IMAGE_NT_HEADERS64);
    (
        nt_headers.FileHeader.NumberOfSections as usize,
        nt_header_ptr + std::mem::size_of::<IMAGE_NT_HEADERS64>(),
    )
}

#[cfg(target_arch = "x86")]
unsafe fn get_nt_headers(nt_header_ptr: usize) -> (usize, usize) {
    let nt_headers = &*(nt_header_ptr as *const IMAGE_NT_HEADERS32);
    (
        nt_headers.FileHeader.NumberOfSections as usize,
        nt_header_ptr + std::mem::size_of::<IMAGE_NT_HEADERS32>(),
    )
}