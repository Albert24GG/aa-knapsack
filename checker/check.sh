#!/bin/bash

# @option -o --output-dir=./output The output directory
# @option -t --test-dir=./tests The directory containing the tests
# @option -a --answers-dir=./answers The directory where the answers are stored
# @flag -s --save Save the output of the tests

available_methods=("bkt" "dp" "fptas")

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

# Run a given test
# $1: the test
# $2: the answer file
# $3: algorithm
# $4: the granularity for fptas (used only for fptas)
run_test() {
    granularity=${4:-1}

    input_file="$argc_test_dir"/"$1".kp
    answer="$(<$2)"

    cargo run -r -- --input-file "$input_file" --granularity "$granularity" run "$3" 2>>debug.log

    exit_code=$?
    if [ $exit_code -ne 0 ]; then
        if [ $exit_code -eq 137 ]; then
            echo "Memory limit exceeded"
        else
            echo "Failed to run the benchmark"
        fi
        return 1
    fi

    # Output saved in out.json
    if [ "$argc_save" ]; then
        full_out_path="$argc_output_dir"/"$2"/"$([ "$2" == "fptas" ] && echo "$1"_out_g"$granularity" || echo "$1"_out)".json
        mkdir -p $(dirname "$full_out_path")
        cp out.json "$full_out_path"
    fi

    optimal_value=$(jq -r '.total_value' out.json)
    items=$(jq -r '.items | @sh' out.json)
    rm out.json

    if [ "$3" == "fptas" ]; then
        # Check how close the value is to the answer
        percentage=$(echo "scale=5; 100 * $optimal_value / $answer" | bc)
        echo "Fptas for granularity $argc_granularity: VALUE is $percentage% of ANSWER"
        return 0
    fi

    if [ "$answer" != "$optimal_value" ]; then
        echo "Expected: $answer"
        echo "Got: $optimal_value"
        echo "Value mismatch"
        return 1
    fi

    xmake r check $optimal_value $items <"$input_file"
    return $?
}

init() {
    xmake f -m release
    xmake clean
    xmake

    cargo build -r

    if [ $? -ne 0 ]; then
        echo "Failed to build the project"
        exit 1
    fi

    # Save the output if specified
    if [ "$argc_save" ]; then
        for method in "${available_methods[@]}"; do
            mkdir -p "$argc_output_dir"/"$method"
        done
    fi
}

# @cmd Run a single test
# @arg test! <TEST>   The test from the specified test directory. Eg: n500/r01000/00Uncorrelated/s000
# @arg method![bkt|dp|fptas] <METHOD>     The method/algorithm to be used
# @option -g --granularity=1  The granularity to be used for fptas
run_one_test() {
    init
    # Precalculate the answer
    input_file="$argc_test_dir"/"$argc_test".kp
    xmake r knapsack_dp <"$input_file" >/tmp/tmp.ans

    if [ "$argc_method" == "fptas" ]; then
        if [ "$argc_granularity" -lt 1 ]; then
            echo "Granularity must be greater than 0"
            exit 1
        fi

        echo "Running $argc_test with $argc_method and granularity $argc_granularity"
    else
        echo "Running $argc_test with $argc_method"
    fi

    run_test "$argc_test" "/tmp/tmp.ans" "$argc_method" "$argc_granularity"

    [ $? -eq 1 ] && echo -e "${RED}FAILED${NC}" || echo -e "${GREEN}PASSED${NC}"
    rm /tmp/tmp.ans

    exit 0
}

precompute_answers() {
    for method in "${available_methods[@]}"; do
        echo "Precalculating answers for $method"

        method_info="$(jq -r ".$method" method_validation.json)"

        tests="$(echo "$method_info" | jq -r ".inputs.[]")"

        for input_test in $tests; do
            ans_path="$argc_answers_dir"/"$input_test".ans

            if [ -f "$ans_path" ]; then
                continue
            fi

            mkdir -p $(dirname "$ans_path")

            xmake r knapsack_dp <"$argc_test_dir"/"$input_test".kp >"$ans_path"
        done

    done

}

# @cmd Run all tests
run_all_tests() {
    init

    precompute_answers

    for method in "${available_methods[@]}"; do
        echo "Running validation for $method"

        method_info="$(jq -r ".$method" method_validation.json)"

        tests="$(echo "$method_info" | jq -r ".inputs.[]")"

        for input_test in $tests; do

            granularities="$(echo "$method_info" | jq -r ".granularities // empty")"
            answer_path="$argc_answers_dir"/"$input_test".ans

            if [ -n "$granularities" ]; then
                for granularity in $(echo "$granularities" | jq -r '.[]'); do
                    echo "Running $input_test with $method and granularity $granularity"
                    run_test "$input_test" "$answer_path" "$method" "$granularity"

                    exit_code=$?
                    if [ $exit_code -ne 0 ]; then
                        break
                    fi
                done
            else
                echo "Running $input_test with $method"
                run_test "$input_test" "$answer_path" "$method"
                exit_code=$?
            fi

            [ $exit_code -ne 0 ] && echo -e "${RED}FAILED${NC}" || echo -e "${GREEN}PASSED${NC}"

        done
    done

    exit 0
}

eval "$(argc --argc-eval "$0" "$@")"
