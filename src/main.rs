mod error;

pub use error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
	println!("Hello, World!");

	Ok(())
}
