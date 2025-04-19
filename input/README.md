# ArkFlow Input Example

This project demonstrates how to create a custom Input plugin for ArkFlow using Rust. This example implements a simple `input_example` that generates a basic message batch periodically.

## Project Overview

Custom Input plugins allow you to integrate various data sources into ArkFlow that are not supported out-of-the-box. You can implement logic to connect to external systems, read data, and format it into `MessageBatch` for downstream processing within ArkFlow.

In this example, we demonstrate how to:

- Implement the `Input` trait for custom data reading logic.
- Implement the `InputBuilder` trait to construct the custom input.
- Register the custom input builder with ArkFlow.

## Dependencies

This project depends on the following libraries:

- `arkflow-core` - ArkFlow core library, providing traits like `Input`, `InputBuilder`, and `MessageBatch`.
- `async-trait` - For using async functions in traits.
- `tokio` - Asynchronous runtime (used here for `sleep`).
- `tracing` - For logging.

## Code Analysis

### Input Implementation (`InputExample`)

```rust
use std::sync::Arc;
use arkflow_core::{Error, MessageBatch};
use arkflow_core::input::{Ack, Input, InputBuilder, NoopAck};
use async_trait::async_trait;
use tracing::info;

struct InputExample;

#[async_trait]
impl Input for InputExample {
    async fn connect(&self) -> Result<(), Error> {
        info!("InputExample::connect");
        Ok(())
    }

    async fn read(&self) -> Result<(MessageBatch, Arc<dyn Ack>), Error> {
        // Simulate reading data with a delay
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        info!("InputExample::read");
        // Create a simple message batch
        Ok((MessageBatch::new_binary(vec!["{}".to_string().into_bytes()])?, Arc::new(NoopAck)))
    }

    async fn close(&self) -> Result<(), Error> {
        info!("InputExample::close");
        Ok(())
    }
}
```

This struct implements the core logic for connecting, reading data (simulated here), and closing the input source.

### Input Builder Implementation (`InputExampleBuilder`)

```rust
struct InputExampleBuilder;
impl InputBuilder for InputExampleBuilder {
    fn build(&self, _: &Option<serde_json::value::Value>) -> Result<Arc<dyn Input>, Error> {
        // Creates an instance of InputExample
        Ok(Arc::new(InputExample))
    }
}
```

This builder is responsible for creating instances of our custom `InputExample`.

### Input Registration

```rust
pub fn init() {
    // Register the builder under the name "input_example"
    arkflow_core::input::register_input_builder("input_example", Arc::new(InputExampleBuilder));
}
```

The `init` function is typically called by ArkFlow when loading the plugin. It registers the `InputExampleBuilder` so that ArkFlow can create instances of `InputExample` when specified in the configuration.

## Usage

1.  Ensure you have Rust and Cargo installed.
2.  Clone the ArkFlow plugin examples repository (if you haven't already):
    ```bash
    git clone https://github.com/arkflow-rs/arkflow-plugin-examples.git
    cd arkflow-plugin-examples
    ```
3. Build the project:
   ```bash
   cargo build
   ```