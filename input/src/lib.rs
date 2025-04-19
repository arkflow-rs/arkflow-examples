use std::sync::Arc;
use arkflow_core::{Error, MessageBatch};
use arkflow_core::input::{Ack, Input, InputBuilder, NoopAck};
use async_trait::async_trait;
use tracing::info;

struct InputEmpty {}

#[async_trait]
impl Input for InputEmpty {
    async fn connect(&self) -> Result<(), Error> {
        // do nothing
        info!("InputEmpty::connect");

        Ok(())
    }

    async fn read(&self) -> Result<(MessageBatch, Arc<dyn Ack>), Error> {
        // Read the data
        info!("InputEmpty::read");
        Ok((MessageBatch::new_binary(vec!["{}".to_string().into_bytes()])?, Arc::new(NoopAck)))
    }

    async fn close(&self) -> Result<(), Error> {
        // do nothing
        info!("InputEmpty::close");
        Ok(())
    }
}

struct InputEmptyBuilder;
impl InputBuilder for InputEmptyBuilder {
    fn build(&self, _: &Option<serde_json::value::Value>) -> Result<Arc<dyn Input>, Error> {
        Ok(Arc::new(InputEmpty {}))
    }
}

pub fn init() {
    arkflow_core::input::register_input_builder("input_empty", Arc::new(InputEmptyBuilder));
}