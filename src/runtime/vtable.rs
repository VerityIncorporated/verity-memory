/// Resolves a vtable from a given raw pointer.
/// 
/// # Safety
/// This function is `unsafe` because it dereferences a raw pointer, which could lead to undefined behavior if the pointer is null or invalid.
/// 
/// # Type Parameters
/// - `T`: The type of the vtable. It must implement the `Copy` trait to allow safe copying.
/// 
/// # Parameters
/// - `vtable_ptr`: A raw pointer to the vtable of type `T`.
/// 
/// # Returns
/// - `T`: The resolved vtable value.
/// 
/// # Panics
/// - This function will panic if the provided `vtable_ptr` is null, indicating an invalid vtable pointer.
/// 
/// # Example
/// ```rust
/// use std::ffi::c_void;
/// use verity_memory::runtime::vtable;
/// 
/// #[repr(C)]
/// #[derive(Clone, Copy)]
/// struct MyVTable {
///     do_something: unsafe extern "C" fn(*mut c_void),
/// }
/// 
/// unsafe extern "C" fn do_something_fn(_: *mut c_void) {
///     println!("Do something!");
/// }
/// 
/// let vtable = MyVTable {
///     do_something: do_something_fn,
/// };
/// let vtable_ptr: *const MyVTable = &vtable;
/// 
/// unsafe { 
///     let resolved_vtable = vtable::resolve_vtable(vtable_ptr);
///     (resolved_vtable.do_something)(std::ptr::null_mut());
/// };
/// ```
pub unsafe fn resolve_vtable<T: Copy>(vtable_ptr: *const T) -> T {
    if vtable_ptr.is_null() {
        panic!("Null pointer to vtable");
    }

    *vtable_ptr
}

/// Resolves a vtable from a double pointer (pointer to a pointer) to the vtable.
/// 
/// # Safety
/// This function is `unsafe` because it dereferences a double raw pointer, which could lead to undefined behavior if any pointer is null or invalid.
/// 
/// # Type Parameters
/// - `T`: The type of the vtable. It must implement the `Copy` trait to allow safe copying.
/// 
/// # Parameters
/// - `vtable_ptr`: A raw pointer to a pointer of type `T`.
/// 
/// # Returns
/// - `T`: The resolved vtable value.
/// 
/// # Panics
/// - This function will panic if the provided `vtable_ptr` is null, indicating an invalid pointer to the vtable.
/// 
/// # Example
/// ```rust
/// use std::ffi::c_void;
/// use verity_memory::runtime::vtable;
/// 
/// #[repr(C)]
/// #[derive(Clone, Copy)]
/// struct MyVTable {
///     do_something: unsafe extern "C" fn(*mut c_void),
/// }
/// 
/// unsafe extern "C" fn do_something_fn(_: *mut c_void) {
///     println!("Do something!");
/// }
/// 
/// let vtable = MyVTable {
///     do_something: do_something_fn,
/// };
/// let vtable_ptr: *const MyVTable = &vtable;
/// let vtable_ptr: *const *const MyVTable = &vtable_ptr;
/// 
/// unsafe { 
///     let resolved_vtable = vtable::resolve_vtable_dp(vtable_ptr);
///     (resolved_vtable.do_something)(std::ptr::null_mut());
/// };
/// ```
pub unsafe fn resolve_vtable_dp<T: Copy>(vtable_ptr: *const *const T) -> T {
    if vtable_ptr.is_null() {
        panic!("Null pointer to vtable");
    }

    *(*vtable_ptr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct MyVTable {
        pub do_something: unsafe extern "C" fn(*mut c_void),
    }

    #[test]
    fn test_resolve_vtable_valid() {
        unsafe {
            extern "C" fn do_something_fn(_: *mut c_void) {
                println!("Do something!");
            }

            let vtable: MyVTable = MyVTable {
                do_something: do_something_fn,
            };

            let vtable_ptr: *const MyVTable = &vtable;

            let resolved_vtable = resolve_vtable(vtable_ptr);

            (resolved_vtable.do_something)(std::ptr::null_mut());
        }
    }

    #[test]
    #[should_panic(expected = "Null pointer to vtable")]
    fn test_resolve_vtable_null() {
        unsafe {
            let vtable_ptr: *const MyVTable = std::ptr::null();

            resolve_vtable(vtable_ptr);
        }
    }

    #[test]
    fn test_resolve_vtable_dp_valid() {
        unsafe {
            extern "C" fn do_something_fn(_: *mut c_void) {
                println!("Do something!");
            }

            let vtable: MyVTable = MyVTable {
                do_something: do_something_fn,
            };

            let vtable_ptr: *const MyVTable = &vtable;
            let vtable_ptr: *const *const MyVTable = &vtable_ptr;

            let resolved_vtable = resolve_vtable_dp(vtable_ptr);

            (resolved_vtable.do_something)(std::ptr::null_mut());
        }
    }

    #[test]
    #[should_panic(expected = "Null pointer to vtable")]
    fn test_resolve_vtable_dp_null() {
        unsafe {
            let vtable_ptr: *const *const MyVTable = std::ptr::null();
            resolve_vtable_dp(vtable_ptr);
        }
    }
}
