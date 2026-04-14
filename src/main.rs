use opendal::Result;
use opendal::Operator;
use opendal::services::Fs;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello OpenDAL!");

    // Create a file system backend
    let builder = Fs::default();
    // Use current directory as the root
    let builder = builder.root(".");

    // Build the OpenDAL operator
    let op = Operator::new(builder)?.finish();

    // Create a directory
    op.create_dir("hello/").await?;
    println!("✓ Created directory: hello/");

    // Write content to a file
    let content = "Hello, World from OpenDAL!";
    op.write("hello/hello.txt", content).await?;
    println!("✓ Wrote file: hello/hello.txt");

    // Read the content back
    let read_content = op.read("hello/hello.txt").await?;
    println!("✓ Read file content: {}", String::from_utf8_lossy(&read_content.to_vec()));

    // Check file exists
    let exists = op.exists("hello/hello.txt").await?;
    println!("✓ File exists: {exists}");

    // Get file metadata
    let meta = op.stat("hello/hello.txt").await?;
    println!("✓ File size: {} bytes", meta.content_length());

    // List directory contents
    let mut entries = op.list("hello/").await?;
    entries.sort_by(|a, b| a.path().cmp(b.path()));
    println!("✓ Directory listing:");
    for entry in entries {
        println!("  - {} ({:?})", entry.path(), entry.metadata().mode());
    }

    // Delete the file (cleanup optional)
    // op.delete("hello/hello.txt").await?;
    // println!("✓ Deleted file: hello/hello.txt");

    Ok(())
}
