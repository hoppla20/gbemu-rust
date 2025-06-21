# Testing

## Getting started

```bash
# run all tests
cargo test

# run unit tests for the module cpu::alu
cargo test --lib cpu::alu

# run all (non-ingored) blargg individual cpu_instrs tests
cargo test --features nogfx --test blargg_cpu_instrs_individual

# run blargg cpu_instrs test 01 without logs
cargo test --features nogfx --test blargg_cpu_instrs_individual --tests 01

# run blargg cpu_instrs test 02 with info log
RUST_LOG=info cargo test --features nogfx --test blargg_cpu_instrs_individual --tests 02

# run blargg cpu_instrs test 03 with release profile
cargo test --profile release --features nogfx --test blargg_cpu_instrs_individual --tests 02
```

Note:
  - **all tests run without compiler optimizations**
  - in order to run with release profile (with compiler optimizations) use the `--profile` argument
    for `cargo test`

## Blargg cpu_instrs tests

The `blargg_cpu_instrs_individual` integration tests are configured to log `cpu::state` events with
`trace` level to `traces/cpu_instrs_XY.log` in the root directory of this project. After running a
certain amount of cycles (determined per test) the tests calls
[gameboy-doctor](https://github.com/robert/gameboy-doctor). This test suite is configured to requirement the `nogfx` feature due to gameboy-doctor's requirement that the graphics register LY
has to return `0x90`.

In addition to the individual tests, the integration test `blargg_cpu_instrs_full` runs
the whole suite, however without gameboy-doctor and without logging to a file. 

## Benchmarks

There is also a benchmark which collects runtime statistics about the `emulator::step` function:

```bash
# run the bench_step benchmark
cargo bench --bench bench_step
```

This command will output statistics about the runtime of the `emulator::step` function. A more detailed
report can be found under `target/criterion/report/index.html`.
