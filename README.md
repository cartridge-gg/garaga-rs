# ED25519 Test Generator

This is a Rust implementation of test generators for the ED25519 elliptic curve. It specifically generates Cairo test files for ECDSA signature verification with ED25519.

## Features

- Generate ECDSA signature verification tests for ED25519 curve

## Building the Generator

```bash
cargo build --release
```

## Usage

The generator provides a command-line interface to generate ED25519 ECDSA tests:

```bash
# Generate ECDSA signature test for ED25519 with default seed
./target/release/garaga ecdsa

# Generate with a specific seed for reproducibility
./target/release/garaga ecdsa --seed 42

# Use a custom output directory
./target/release/garaga ecdsa --out-dir path/to/output
```

## Output

By default, the test is written to a `tests/ecdsa_ED25519_test.cairo` file relative to the current working directory. You can specify a different output directory using the `--out-dir` option.

## Integration with Cairo

The generated test is designed to be included in the Garaga Cairo project for testing ECDSA signature verification with the ED25519 curve. 

To use the generated test:

1. Generate the test file using this tool
2. Copy the file to the appropriate test directory in your Garaga project
3. Run the tests using Scarb or your preferred Cairo test runner

## Note

This is a focused implementation specifically for generating ED25519 ECDSA tests for the Garaga project, ported from the original Python implementation.
