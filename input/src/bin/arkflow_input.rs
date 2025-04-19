use arkflow_core::cli::Cli;
use arkflow_plugin::{buffer, input, output, processor};
use crate::input as input_example;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    input::init();
    output::init();
    processor::init();
    buffer::init();
    input_example::init();
    let mut cli = Cli::default();
    cli.parse()?;
    cli.run().await
}
