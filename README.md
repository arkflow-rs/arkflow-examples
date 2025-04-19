# ArkFlow Examples

This repository contains a collection of examples demonstrating various features and use cases of the [ArkFlow](https://github.com/arkflow-rs/arkflow) framework.

## Examples

Currently, the following examples are available:

*   **[SQL UDF](./sql_udf/README.md)**: Demonstrates how to create and register a custom User Defined Function (UDF) for use in SQL queries within ArkFlow.

## Getting Started

### Prerequisites

*   Rust and Cargo: Ensure you have a recent version of Rust installed. You can install it from [rustup.rs](https://rustup.rs/).

### Building and Running Examples

1.  Clone this repository:
    ```bash
    git clone https://github.com/arkflow-rs/arkflow-examples.git
    cd arkflow-examples
    ```

2.  Navigate to a specific example directory (e.g., `sql_udf`) and follow the instructions in its `README.md` file.

3.  Generally, you can build an example using Cargo:
    ```bash
    # Replace <example_package_name> with the actual package name (e.g., sql_udf)
    cargo build --package <example_package_name>
    ```

    Refer to the specific example's `README.md` for detailed build and run instructions.

## License

This project is licensed under the Apache-2.0 License. See the [LICENSE](./LICENSE) file for details.

## Contributing

Contributions are welcome! Please refer to the main [ArkFlow repository](https://github.com/arkflow-rs/arkflow) for contribution guidelines, reporting issues, or submitting pull requests.