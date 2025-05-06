# Testing

## Getting started

```bash
# run all tests
cargo test

# run unit tests for the module cpu::alu
cargo test --lib cpu::alu

# run all (non-ingored) blargg cpu_instrs tests
cargo test --tests cpu::tests::blargg

# run blargg cpu_instrs test 01 without logs
cargo test --tests cpu::tests::blargg::test_blargg_cpu_instrs_01

# run blargg cpu_instrs test 02 with info log
RUST_DEBUG=info cargo test --tests test_blargg_cpu_instrs_02
```

Note:
  - **all tests run without compiler optimizations**
  - in order to run with release profile (with compiler optimizations) use the `--profile` argument
    for `cargo test`

## Blargg cpu_instrs tests

The `cpu::tests::blargg::test_cpu_instrs_XY` are configured to log `cpu::state` events with `trace`
level to `traces/cpu_instrs_XY.log` in the root directory of this project. After running a certain
amount of cycles (determined per test) the tests calls
[gameboy-doctor](https://github.com/robert/gameboy-doctor).

In addition to the individual tests, the test `cpu::tests::blargg::test_cpu_instrs_full` runs
the whole suite, however without gameboy-doctor and without logging to a file. The full test is marked
as ignored because all individual tests are already in their own test function. You can however run
the full test manually:

```bash
# run full blarg cpu_instrs test with info logs
RUST_DEBUG=info cargo test --tests cpu::tests::blargg::test_cpu_instrs_full -- --include-ignored
```

## Benchmarks

There is also a benchmark which collects runtime statistics about the `emulator::step` function:

```bash
# run the bench_step benchmark
cargo bench --bench bench_step
```

This command will output statistics about the runtime of the `emulator::step` function. A more detailed
report can be found under `target/criterion/report/index.html`.
