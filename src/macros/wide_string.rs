#[macro_export]
macro_rules! w {
    ($text:literal $(, $args:expr)*) => {{
        let formatted = format!($text $(, $args)*);
        let os_str = std::ffi::OsStr::new(&formatted);
        let wide_string: Vec<u16> = std::os::windows::ffi::OsStrExt::encode_wide(os_str)
            .chain(Some(0))
            .collect();
        wide_string.as_ptr()
    }};
    
    ($text:expr) => {{
        let formatted = $text.to_string();
        let os_str = std::ffi::OsStr::new(&formatted);
        let wide_string: Vec<u16> = std::os::windows::ffi::OsStrExt::encode_wide(os_str)
            .chain(Some(0))
            .collect();
        wide_string.as_ptr()
    }};
}