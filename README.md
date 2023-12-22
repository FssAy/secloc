# SecLoc

SecLoc is a Rust library designed for locating and working with sections in the current executable. It provides a straightforward and efficient way to interact with PE (Portable Executable) sections in Rust applications.

This crate might be useful when trying to read embedded data in the current executable.

## Features

- **Locate PE Sections**: Easily find and work with sections in the current executable.
- **Section Representation**: Represents a PE section with properties like section number, name, virtual address, and data size.
- **Iterate Over Sections**: Provides functionality to iterate over all PE sections and apply a callback function to each.

## Getting Started

To use SecLoc in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
secloc = "0.1.0"
```

## Usage

Here's a quick example to get you started:

```rust
use secloc::SecLoc;

unsafe {
    let secloc = SecLoc::new();
    let sections = secloc.get_all();

    for section in sections {
        println!("Section: {}", section.name);
    }
}
```

## License

SecLoc is licensed under the GPL-3.0-only license.
