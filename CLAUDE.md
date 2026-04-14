# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

- Build: `cargo build`
- Run the example: `cargo run`
- Check for errors without building: `cargo check`
- Run clippy for linting: `cargo clippy`
- Format code: `cargo fmt`
- Clean build artifacts: `cargo clean`

## Architecture

This is a simple Hello World example project for Apache OpenDAL written in Rust.

- **Entry point**: `src/main.rs` - A single binary that demonstrates basic OpenDAL operations with the local filesystem backend
- **Project config**: `Cargo.toml` - Uses OpenDAL 0.52 with `services-fs` (filesystem backend) and Tokio async runtime

## OpenDAL Example

The example demonstrates the core OpenDAL API:

1. `Operator::new(builder)?.finish()` - Initialize the OpenDAL operator from a service builder
2. `create_dir(path)` - Create a directory
3. `write(path, content)` - Write bytes to a file
4. `read(path)` - Read bytes from a file into a Buffer
5. `exists(path)` - Check if a path exists
6. `stat(path)` - Get metadata (content length, mode, etc.)
7. `list(path)` - List entries in a directory
8. `delete(path)` - Delete a file (commented out for demonstration)

OpenDAL provides a consistent API across many storage services - the same code works for S3, GCS, Azure Blob, FTP, and many others by just changing the service builder.

## Important Notes

- The `hello/` directory is created at runtime and its contents are ignored by git
- OpenDAL feature flags: All services are under the `services-*` prefix, so enabling a new storage service just requires adding the corresponding feature
