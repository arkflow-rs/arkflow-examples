use arkflow_core::cli::Cli;
use input as input_example;
use arkflow_plugin::{buffer, input as stable_input, output, processor};
use sql_udf as sql_udf_example;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    stable_input::init();
    output::init();
    processor::init();
    buffer::init();

    input_example::init();
    sql_udf_example::init();
    let mut cli = Cli::default();
    cli.parse()?;
    cli.run().await
}
