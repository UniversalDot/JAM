# Join-Accumulate Machine (Jam) in Rust

This project provides a simple implementation of the Join-Accumulate Machine (Jam) in Rust, based on the concepts described in the graypaper. The implementation includes basic structures for smart contracts, core-time management, and an execution environment.

## Overview

The Join-Accumulate Machine (Jam) is a virtual machine designed to execute smart contracts with a focus on efficient core-time management. This project demonstrates the core components of Jam in a simplified form.

## Features

- **Smart Contract Structure**: Defines basic smart contract structure with code and state.
- **Core-Time Management**: Manages core-time allocation for contract execution.
- **Asynchronous Execution**: Uses `tokio` for asynchronous execution of smart contracts.

## Dependencies

Ensure you have the following dependencies in your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Getting Started

### Prerequisites

- Install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Installation

1. **Clone the repository:**

    ```bash
    git clone https://github.com/UniversalDot/JAM.git
    cd JAM
    ```

2. **Run the application:**

    ```bash
    cargo run
    ```

