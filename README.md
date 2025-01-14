# Knapsack algorithms

## Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed
- [xmake](https://xmake.io/#/) (optional for running the checker script)
- [argc](https://github.com/sigoden/argc) (optional for running the checker and benchmark scripts)

### Build

```bash
cargo build --release
```

## Usage

After building the project, the program can be run alone through the command line:

```bash
cargo run -r -- -h
Usage: knapsack [OPTIONS] --input-file <TEST_FILE> <ACTION> <METHOD>

Arguments:
  <ACTION>  Action to perform [possible values: run, benchmark]
  <METHOD>  Method used for solving the problem [possible values: dp, bkt, fptas]

Options:
  -i, --input-file <TEST_FILE>
  -o, --output-file <OUTPUT_FILE>  [default: out.json]
  -g, --granularity <GRANULARITY>  Granularity for the FPTAS method. This is only used when the method is FPTAS [default: 1]
  -h, --help                       Print help

```

Optionally, the scripts from `checker/` directory can be used for checking the corectness of the program on different test cases and for benchmarking the performance of the program.

```bash
./check.sh -h
USAGE: check [OPTIONS] <COMMAND>

OPTIONS:
  -o, --output-dir <OUTPUT-DIR>    The output directory [default: ./output]
  -t, --test-dir <TEST-DIR>        The directory containing the tests [default: ./tests]
  -a, --answers-dir <ANSWERS-DIR>  The directory where the answers are stored [default: ./answers]
  -c, --config <CONFIG>            The configuration file for the tests [default: ./validation_conf.json]
  -s, --save                       Save the output of the tests
  -h, --help                       Print help
  -V, --version                    Print version

COMMANDS:
  run_one_test   Run a single test [aliases: run-one-test]
  run_all_tests  Run all tests [aliases: run-all-tests]
```

```bash
./benchmark.sh -h
USAGE: bench [OPTIONS] <COMMAND>

OPTIONS:
  -b, --benchmarks-dir <BENCHMARKS-DIR>  The directory where the benchmarks are stored [default: ./benchmarks]
  -t, --test-dir <TEST-DIR>              The directory containing the tests [default: ./tests]
  -c, --config <CONFIG>                  The configuration file for the benchmarks [default: ./benchmark_conf.json]
  -h, --help                             Print help
  -V, --version                          Print version

COMMANDS:
  benchmark_one_test  Benchmark a single test [aliases: benchmark-one-test]
  benchmark_all       Run all benchmarks [aliases: benchmark-all]
```

The configuration for this scripts is done through a combination of cli arguments and the configuration file. A sample can be found in `checker/config_model.json`.

## Examples

```bash
cargo run -r -- -i checker/tests/n5/r01000/02StronglyCorrelated/s000.kp run dp
bat out.json
───────┬────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
       │ File: out.json
───────┼────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
   1   │ {
   2   │   "items": [4, 0],
   3   │   "total_value": 1557
   4   │ }
───────┴────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
```

```bash
cargo run -r -- -i checker/tests/n5/r01000/02StronglyCorrelated/s000.kp benchmark dp
bat out.json
───────┬────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
       │ File: out.json
───────┼────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
   1   │ {
   2   │   "mean": {
   3   │     "confidence_interval": {
   4   │       "confidence_level": 0.95,
   5   │       "lower_bound": 15206,
   6   │       "upper_bound": 15248
   7   │     },
   8   │     "point_estimate": 15227,
   9   │     "standard_error": 10.699105407028291
  10   │   },
  11   │   "median": {
  12   │     "confidence_interval": {
  13   │       "confidence_level": 0.95,
  14   │       "lower_bound": 15200,
  15   │       "upper_bound": 15257
  16   │     },
  17   │     "point_estimate": 15226,
  18   │     "standard_error": 20.55481501245556
  19   │   },
  20   │   "std_dev": {
  21   │     "confidence_interval": {
  22   │       "confidence_level": 0.95,
  23   │       "lower_bound": 23,
  24   │       "upper_bound": 43
  25   │     },
  26   │     "point_estimate": 35,
  27   │     "standard_error": 5.166250454263611
  28   │   }
  29   │ }
───────┴────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
```

## Tests

The tests used can be found in the `tests/` directory. These are taken from the [kplib](https://github.com/likr/kplib) repository.
