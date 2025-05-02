# Testing

## Local machine

Examples:

```bash
# run all tests
cargo test

# run unit tests for the module cpu::alu
cargo test --lib cpu::alu

# run blargg cpu_instrs 01
# Note: blargg tests are ignored until they pass
cargo test --tests test_blargg_cpu_instrs_01 -- --include-ignored
```
