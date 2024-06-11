# Rz

**Rz** or **Rzip** is a high-performance, secure compression and decompression tool developed in [Rust](https://rust-lang.org/). Designed for efficiency and safety, Rzip provides a powerful CLI for managing your compressed files with ease and reliability.

## Features

- **Speed**: Utilizes Rust's optimizations to ensure fast compression and decompression operations.
- **Security:** Built with Rust's strong safety features to minimize errors and vulnerabilities.
- **CLI:** User-friendly and straightforward command-line interface for easy file management.
- **Versatile:** Supports various file formats and compression algorithms for flexibility.

### Compatibility

| Compression Method | Compression | Decompression |
| ------------------ | ----------- | ------------- |
| Stored             | ✓           | ✓             |
| Deflate            | ✓           | ✓             |
| Deflate64          |             | ✓             |
| Bzip2              | ✓           |               |
| LZMA               |             | ✓             |
| Zstd               | ✓           |               |

This means that Rzip supports the following extensions:
`.zip`, `.bz2`, `.tbz2`, `.lzma`, `.zst`.

## CLI

**Usage:** `rz <command> [options] <args>`

### Commands

| Command | Description | Arguments | Alias |
| ------- | ----------- | --------- | ----- |
| `append` | Appends a file to an existing archive | `<sources...> <dest.zip>` | `a` |
| `compress` | Compresses a file or directory |`<sources...> <dest.zip>` | `c` |
| `extract` | Decompresses a file | `<source.zip> <dest>` | `x` |

### Options

| Option | Alias | Type | Description | Required | On command |
| ------ | ----- | ---- | ----------- | -------- | ---------- |
| `--level` | `-l` | Integer(64) | Compression level | No | `*` |
| `--method` | `-m` | Enum | Compression method (stored/deflate/etc...) | No | `*` |
| `--pick` | `-p` | String | File to extract from archive | No | `extract` |
| `--unix_permissions` | `-u` | UInteger(32) | Unix permissions for new files | No | `*` |

> Note that the `*` symbol indicates that the option is available for all commands.
> Global options must be used before the command.

## Contributing

Contributions are welcome! If you wish to contribute to Rzip, please follow these steps:

1. Fork the repository.
2. Create a new branch (git checkout -b feature/new-feature).
3. Make your changes and commit them (git commit -m 'feature: 🌱 new feature').
4. Push your changes (git push origin feature/new-feature).
5. Open a Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Rust](https://rust-lang.org/)
- [Clap-rs](https://github.com/clap-rs/clap)
- [Zip-rs](https://github.com/zip-rs/zip2)

Developed with ❤️ by [Sammwy](https://github.com/sammwyy).
