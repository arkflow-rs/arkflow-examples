use std::sync::Arc;
use arkflow_core::{Error, MessageBatch};
use arkflow_core::input::{Ack, Input, InputBuilder, NoopAck};
use async_trait::async_trait;
use tracing::info;

struct InputExample;

#[async_trait]
impl Input for InputExample {
    async fn connect(&self) -> Result<(), Error> {
        // do nothing
        info!("InputEmpty::connect");

        Ok(())
    }

    async fn read(&self) -> Result<(MessageBatch, Arc<dyn Ack>), Error> {
        // Read the data
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        info!("InputEmpty::read");
        Ok((MessageBatch::new_binary(vec!["{}".to_string().into_bytes()])?, Arc::new(NoopAck)))
    }

    async fn close(&self) -> Result<(), Error> {
        // do nothing
        info!("InputEmpty::close");
        Ok(())
    }
}

struct InputExampleBuilder;
impl InputBuilder for InputExampleBuilder {
    fn build(&self, _: &Option<serde_json::value::Value>) -> Result<Arc<dyn Input>, Error> {
        Ok(Arc::new(InputExample))
    }
}

pub fn init() {
    arkflow_core::input::register_input_builder("input_example", Arc::new(InputExampleBuilder));
}