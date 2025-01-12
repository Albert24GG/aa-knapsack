#!/bin/bash

# @option -b --benchmarks-dir=./benchmarks The directory where the benchmarks are stored
# @option -t --test-dir=./tests The directory containing the tests
# @option -c --config=./benchmark_conf.json The configuration file for the benchmarks

available_methods=("bkt" "dp" "fptas")

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

# Benchmark a given test
# $1: the test
# $2: algorithm
# $3: the granularity for fptas (used only for fptas)
benchmark_test() {
    granularity=${3:-1}

    input_file="$argc_test_dir"/"$1".kp
    benchmark_out_file="$argc_benchmarks_dir"/"$2"/"$([ "$2" == "fptas" ] && echo "$1"_bench_g"$granularity" || echo "$1"_bench)".json

    mkdir -p $(dirname "$benchmark_out_file")
    cargo run -r -- --input-file "$input_file" --granularity "$granularity" benchmark "$2" 2>>debug.log

    exit_code=$?
    if [ $exit_code -ne 0 ]; then
        if [ $exit_code -eq 137 ]; then
            echo "Memory limit exceeded" | tee "$benchmark_out_file"
        else
            echo "Failed to run the benchmark" | tee "$benchmark_out_file"
        fi
        return 1
    fi

    mv out.json "$benchmark_out_file"
    return 0
}

init() {
    cargo build -r

    if [ $? -ne 0 ]; then
        echo "Failed to build the project"
        exit 1
    fi

    if [ ! -d "$argc_benchmarks_dir" ]; then
        for method in "${available_methods[@]}"; do
            mkdir -p "$argc_benchmarks_dir"/"$method"
        done

    fi
}

# @cmd Benchmark a single test
# @arg test! <TEST>   The test from the specified test directory. Eg: n500/r01000/00Uncorrelated/s000
# @arg method![bkt|dp|fptas] <METHOD>     The method/algorithm to be used
# @option -g --granularity=1  The granularity to be used for fptas
benchmark_one_test() {
    init

    if [ "$argc_method" == "fptas" ]; then
        echo "Running $argc_test with $argc_method and granularity $argc_granularity"
    else
        echo "Running $argc_test with $argc_method"
    fi

    benchmark_test "$argc_test" "$argc_method" "$argc_granularity"

    [ $? -eq 1 ] && echo -e "${RED}FAILED${NC}" || echo -e "${GREEN}PASSED${NC}"

    exit 0
}

# @cmd Run all benchmarks
benchmark_all() {
    init

    for method in "${available_methods[@]}"; do
        echo "Running benchmark for $method"

        method_info="$(jq -r ".$method" $argc_config)"

        tests="$(echo "$method_info" | jq -r ".inputs.[]")"

        for input_test in $tests; do

            granularities="$(echo "$method_info" | jq -r ".granularities // empty")"

            if [ -n "$granularities" ]; then
                for granularity in $(echo "$granularities" | jq -r '.[]'); do
                    echo "Running $input_test with $method and granularity $granularity"

                    benchmark_test "$input_test" "$method" "$granularity"

                    exit_code=$?
                    if [ $exit_code -ne 0 ]; then
                        break
                    fi
                done
            else
                echo "Running $input_test with $method"

                benchmark_test "$input_test" "$method"

                exit_code=$?
            fi

            [ $exit_code -ne 0 ] && echo -e "${RED}FAILED${NC}" || echo -e "${GREEN}PASSED${NC}"

        done
    done

    exit 0
}

eval "$(argc --argc-eval "$0" "$@")"
