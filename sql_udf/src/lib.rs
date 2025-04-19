use std::sync::Arc;
use arkflow_plugin::processor;
use datafusion::arrow::array::{ArrayRef, Int64Array};
use datafusion::arrow::datatypes::DataType;
use datafusion::common::cast::as_int64_array;
use datafusion::logical_expr::{ColumnarValue, Volatility};
use datafusion::common::Result;
use datafusion::prelude::create_udf;


pub fn init() {
    let udf = create_udf(
        "add_one",
        vec![DataType::Int64],
        DataType::Int64,
        Volatility::Immutable,
        Arc::new(add_one),
    );
    processor::udf::scalar_udf::register(udf);
}


fn add_one(args: &[ColumnarValue]) -> Result<ColumnarValue> {
    // Error handling omitted for brevity
    let args = ColumnarValue::values_to_arrays(args)?;
    let i64s = as_int64_array(&args[0])?;

    let new_array = i64s
        .iter()
        .map(|array_elem| array_elem.map(|value| value + 1))
        .collect::<Int64Array>();

    Ok(ColumnarValue::from(Arc::new(new_array) as ArrayRef))
}
