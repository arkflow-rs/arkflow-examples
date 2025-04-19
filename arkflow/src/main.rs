use arkflow_core::cli::Cli;
use input as input_example;
use arkflow_plugin::{buffer, input as stable_input, output, processor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    stable_input::init();
    output::init();
    processor::init();
    buffer::init();

    input_example::init();
    sql_udf::init();
    let mut cli = Cli::default();
    cli.parse()?;
    cli.run().await
}
