#[cfg(feature = "advanced-write")]
pub mod asm;
pub mod read;
pub mod write;

pub use read::read_memory;
pub use write::write_memory;

#[cfg(feature = "advanced-write")]
pub use write::nop_instructions;
#[cfg(feature = "advanced-write")]
pub use write::replace_return_value;