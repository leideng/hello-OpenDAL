# hello-OpenDAL

Hello World project for [Apache OpenDAL](https://opendal.apache.org/) - One Layer, All Storage.

This is a simple Rust project demonstrating the basic usage of OpenDAL with the file system backend.

## What is OpenDAL?

OpenDAL is a data access layer that allows you to interact with many different storage services (like S3, Google Cloud Storage, Azure Blob, local filesystem, etc.) using a consistent API.

## Prerequisites

- Rust toolchain (1.75+ recommended)

## Running the example

```bash
cargo run
```

## What the example does

1. Creates a directory `hello/`
2. Writes "Hello, World from OpenDAL!" to `hello/hello.txt`
3. Reads the content back from the file
4. Checks if the file exists
5. Gets file metadata (size)
6. Lists the contents of the directory

## Dependencies

- `opendal`: The OpenDAL library with filesystem backend enabled
- `tokio`: Async runtime for async operations

## Learn More

- [OpenDAL Documentation](https://opendal.apache.org/docs/)
- [OpenDAL GitHub Repository](https://github.com/apache/opendal)

