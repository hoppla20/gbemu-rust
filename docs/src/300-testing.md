# Testing

## Local machine

Examples:

```bash
# run all tests
cargo test

# run unit tests for the module cpu::alu
cargo test --lib cpu::alu

# run blargg cpu_instrs 01
cargo test --tests test_blargg_cpu_instrs_01

# run ignored test blargg cpu_instrs 02 with debug log
RUST_DEBUG=debug cargo test --tests test_blargg_cpu_instrs_02 -- --include-ignored
```
