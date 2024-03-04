# Python vs. Rust (PyO3) Performance Benchmark

This repository contains a simple yet effective benchmark comparing the performance of pure Python code against Python integrated with Rust using PyO3. The benchmark focuses on calculating the value of Pi using a series of terms in a series, a task that is computationally intensive and serves well to illustrate performance differences between the two approaches.

## Objective

The primary objective of this benchmark is to showcase the potential performance improvements that can be achieved by integrating Rust into Python projects for computationally intensive tasks. This is demonstrated through the calculation of Pi to a significant number of terms, a process that benefits from Rust's performance advantages.

## Prerequisites

To run this benchmark on your local machine, you need to have the following installed:

- Python 3.8 or higher
- Rust and Cargo (See [Rust's installation guide](https://www.rust-lang.org/tools/install))
- `maturin` for building and publishing Rust-based Python packages (Install via Cargo with `cargo install maturin`)

Additionally, ensure all Python dependencies are installed by running:

```bash
pip install -r requirements.txt
```

## Installation

First, you'll need to compile the Rust code. Navigate to the directory containing the Rust project and run:

```bash
poetry install
maturin develop
```

This command builds the Rust code and makes it accessible to Python via PyO3.

## Usage

To run the benchmark, simply execute the main Python script:

```bash
poetry run python benchmark/main.py
```

This script will calculate Pi using both the pure Python function and the Rust-enhanced Python function, outputting the time taken by each method to the console.

## Benchmark Results

Below is an example of the output you might see after running the benchmark (Note: Actual timings will vary based on the hardware and system load):

```plaintext
calculate_pi_with_python executed in 9.44 seconds.
π = 3.141592643589326 - Python
calculate_pi_with_pyo3 executed in 0.65 seconds.
π = 3.141592643589326 - PyO3
```

As observed, the Rust-enhanced Python function (`calculate_pi_with_pyo3`) executes significantly faster than its pure Python counterpart, showcasing the potential benefits of integrating Rust into Python projects for performance-critical tasks.
