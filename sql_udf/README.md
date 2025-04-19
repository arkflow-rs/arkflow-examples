# SQL UDF (User Defined Function) Example

This project demonstrates how to create SQL User Defined Functions (UDF) for ArkFlow using Rust. This example implements a simple `add_one` function that takes an Int64 type input and returns the result of adding 1 to that value.

## Project Overview

User Defined Functions (UDF) allow you to extend SQL functionality, implementing custom logic that standard SQL functions cannot directly provide. In this example, we demonstrate how to:

- Create a simple UDF function
- Register the function to the ArkFlow processing engine
- Use the function in SQL queries

## Dependencies

This project depends on the following libraries:

- `arkflow-core` - ArkFlow core library
- `arkflow-plugin` - ArkFlow plugin system
- `datafusion` - High-performance query engine for SQL processing
- `tokio` - Asynchronous runtime

## Code Analysis

### Function Implementation

```rust
fn add_one(args: &[ColumnarValue]) -> Result<ColumnarValue> {
    // Convert parameters to arrays
    let args = ColumnarValue::values_to_arrays(args)?;
    let i64s = as_int64_array(&args[0])?;

    // Add 1 to each element
    let new_array = i64s
        .iter()
        .map(|array_elem| array_elem.map(|value| value + 1))
        .collect::<Int64Array>();

    Ok(ColumnarValue::from(Arc::new(new_array) as ArrayRef))
}
```

### Function Registration

```rust
pub fn init() {
    let udf = create_udf(
        "add_one",           // Function name
        vec![DataType::Int64], // Parameter type
        DataType::Int64,       // Return type
        Volatility::Immutable, // Function stability
        Arc::new(add_one),     // Function implementation
    );
    processor::udf::scalar_udf::register(udf);
}
```

## Usage

1. Ensure you have Rust and Cargo installed
2. Clone the ArkFlow examples repository:
   ```bash
   git clone https://github.com/arkflow-rs/arkflow-examples.git
   cd arkflow-examples
   ```
3. Build the project:
   ```bash
   cargo build --package sql_udf
   ```

## Using in SQL

Once the UDF is registered, you can use it in SQL queries just like built-in functions:

```sql
SELECT add_one(column_name) FROM your_table;
```

## Extended Examples

You can create more complex UDFs based on this example:

- Process multiple input parameters
- Return different data types
- Implement more complex business logic

## License

This project is licensed under the Apache-2.0 License. See the [LICENSE](https://github.com/arkflow-rs/arkflow/blob/main/LICENSE) file for details.

## Contributing

Issues and pull requests are welcome to the [ArkFlow repository](https://github.com/arkflow-rs/arkflow).