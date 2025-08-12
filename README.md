# Zipper CLI

A fast, lightweight command-line ZIP archiver written in Rust. Create and extract ZIP archives with minimal dependencies and optimized binary size.

## Features

- 🚀 **Fast**: Written in Rust for optimal performance
- 📦 **Lightweight**: Minimal dependencies and optimized binary size
- 🛠️ **Simple**: Easy-to-use CLI interface
- 🔒 **Safe**: Memory-safe with Rust's ownership system
- 📁 **Flexible**: Support for both files and directories

## Installation

### From Source

1. Clone the repository:
```bash
git clone https://github.com/masoom369/zipper.git
cd zipper
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release/zipper`

### Using Cargo

If you have Rust installed, you can install directly with Cargo:

```bash
cargo install --git https://github.com/masoom369/zipper.git
```

## Usage

### Creating ZIP Archives

Create a ZIP archive from a file:
```bash
zipper create archive.zip file.txt
```

Create a ZIP archive from a directory:
```bash
zipper create archive.zip ./my-directory
```

### Extracting ZIP Archives

Extract a ZIP archive to a directory:
```bash
zipper extract archive.zip ./output-directory
```

### Help

Get help information:
```bash
zipper --help
```

Get help for specific commands:
```bash
zipper create --help
zipper extract --help
```

## Examples

### Archive a project directory
```bash
zipper create my-project.zip ./my-project
```

### Extract a downloaded archive
```bash
zipper extract download.zip ./extracted-files
```

### Archive a single file
```bash
zipper create document.zip important-file.pdf
```

## Project Structure

```
rust_zipper_cli/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   └── main.rs         # Main application code
└── README.md           # This file
```

## Dependencies

- **zip**: ZIP file creation and extraction
- **clap**: Command-line argument parsing

## Building

### Development Build
```bash
cargo build
```

### Release Build (Optimized)
```bash
cargo build --release
```

### Run Tests
```bash
cargo test
```

## Performance

The project is optimized for minimal binary size with the following release profile settings:
- Link Time Optimization (LTO) enabled
- Debug symbols stripped
- Optimized for size (`opt-level = 'z'`)
- Overflow checks maintained for safety

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Roadmap

- [ ] Add password protection support
- [ ] Add compression level options
- [ ] Add progress indicators for large files
- [ ] Add support for other archive formats
- [ ] Add recursive directory compression options

## Issues

If you encounter any issues or have feature requests, please [open an issue](https://github.com/masoom369/zipper/issues) on GitHub.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [zip-rs](https://github.com/zip-rs/zip) for ZIP functionality
- Uses [clap](https://github.com/clap-rs/clap) for CLI parsing
