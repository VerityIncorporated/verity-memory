# Verity Memory

Personal memory library with some cool features.

## Features

- **AOB Pattern Scanning**: Locate unique memory patterns efficiently.
- **Nop Instructions**: Replace machine instructions with `NOP` (no operation) to neutralize code segments.
- **Replace Return Values**: Intercept and replace function return values for integers.

## Getting Started

Add Verity Memory to your `Cargo.toml`:

```toml
[dependencies]
verity-memory = "<latest_version>"
```

## Usage Examples

### 1. AOB Pattern Scanning

Scan memory for a unique sequence of bytes (Array of Bytes):

```rust
use verity_memory::aob;

fn main() {
    let pattern = aob::scan_unique("FF 08 8D 44 24 1C");
    let address = match pattern {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            return;
        }
    };

    println!("Found pattern at address: {:X}", address);
}
```

### 2. Nop Instructions

Neutralize code by overwriting instructions with `NOP`:

```rust
use verity_memory::write;

fn main() {
    let address = 0x12345678; // Replace with the actual address

    // Replace 1 instruction with NOP
    write::nop_instructions(address, 1);

    println!("Instruction at {:X} has been neutralized.", address);
}
```

### 3. Replace Return Value

Intercept and replace the return value of a function (integers only):

```rust
use verity_memory::write;

fn main() {
    let ptr_i32: usize = 0x12345678; // Replace with the actual function pointer

    if let None = write::replace_return_value::<i32>(ptr_i32, Some(1)) {
        eprintln!("Failed to write i32 return value");
    } else {
        println!("Successfully replaced return value at {:X}.", ptr_i32);
    }
}
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request to enhance the library.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

**Note**: This library operates on low-level memory manipulation and should be used responsibly. Ensure you have the necessary permissions to modify the target process or application.

