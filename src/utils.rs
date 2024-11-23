use std::ptr::null_mut;

use winapi::um::libloaderapi::GetModuleHandleW;

use crate::w;

pub(crate) fn check_alignment<T>(ptr: *const T) -> bool {
    if ptr.is_null() {
        return false;
    }
    let alignment = std::mem::align_of::<T>();
    (ptr as usize) % alignment == 0
}

pub unsafe fn import_function<'a, F>(
    dll_name: &str,
    proc_name: &str,
) -> Option<(
    libloading::os::windows::Library,
    libloading::os::windows::Symbol<F>,
)>
where
    F: Sized,
{
    match libloading::os::windows::Library::new(dll_name) {
        Ok(lib) => {
            let proc_name_c = std::ffi::CString::new(proc_name).unwrap();
            match lib.get::<F>(proc_name_c.as_bytes_with_nul()) {
                Ok(symbol) => Some((lib, symbol)),
                Err(_) => {
                    println!("Failed to get function address for {}", proc_name);
                    None
                }
            }
        }
        Err(_) => {
            println!("Failed to load DLL: {}", dll_name);
            None
        }
    }
}

pub fn module_base(module_name: Option<&str>) -> *mut u8 {
    unsafe {

        let handle = match module_name {
            Some(name) => {
                GetModuleHandleW(w!(name))
            }
            None => GetModuleHandleW(null_mut()),
        };

        if handle.is_null() {
            panic!("Failed to get module handle");
        }
        handle as *mut u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_alignment() {
        let x: u8 = 42;
        let ptr = &x as *const u8;
        assert!(check_alignment(ptr));

        let y: i32 = 42;
        let ptr = &y as *const i32;
        assert!(check_alignment(ptr));
    }

    #[test]
    fn test_check_alignment_unaligned() {
        let x: u8 = 42;
        let ptr = &x as *const u8;
        assert!(check_alignment(ptr));
    }

    #[test]
    fn test_import_function_fail_load() {
        let result = unsafe { import_function::<fn()>("non_existent_dll.dll", "non_existent_function") };
        assert!(result.is_none());
    }

    #[test]
    fn test_import_function_fail_get() {
        let result = unsafe { import_function::<fn()>("kernel32.dll", "non_existent_function") };
        assert!(result.is_none());
    }

    #[test]
    fn test_import_function_success() {
        let result = unsafe { import_function::<fn()>("kernel32.dll", "GetCurrentProcess") };
        assert!(result.is_some());
    }
}