#!/bin/bash

# @option -o --output-dir=./output The output directory
# @option -t --test-dir=./tests The directory containing the tests
# @option -a --answers-dir=./answers The directory where the answers are stored
# @option -g --granularity=1  The granularity to be used for fptas
# @flag -s --save Save the output of the tests

# Run the tests
# $1: test file
# $2: algorithm
run_test() {
    ANSWER=$(cat "$argc_answers_dir"/$(basename "$1" .kp).ans)
    cargo run -r -- --input-file "$1" --granularity "$argc_granularity" run "$2" 2>>debug.log
    # Output saved in out.json
    if [ "$argc_save" ]; then
        cp out.json "$argc_output_dir"/"$2"/$(basename "$1" .kp).json
    fi
    VALUE=$(jq -r '.total_value' out.json)
    ITEMS=$(jq -r '.items | @sh' out.json)
    rm out.json

    if [ "$2" == "fptas" ]; then
        # Check how close the value is to the answer
        PERCENTAGE=$(echo "scale=5; 100 * $VALUE / $ANSWER" | bc)
        echo "Fptas for granularity $argc_granularity: VALUE is $PERCENTAGE% of ANSWER"
        return 0
    fi

    if [ "$ANSWER" != "$VALUE" ]; then
        echo "Expected: $ANSWER"
        echo "Got: $VALUE"
        echo "Value mismatch"
        return 1
    fi

    xmake r check $VALUE $ITEMS <"$1"
    return $?
}

run_special_tests() {
    echo
    echo "Running special tests (fptas only)"
    echo

    for test in "$argc_test_dir"/special/*.kp; do
        echo "Running $(basename "$test")"
        run_test "$test" "fptas"
        if [ $? -eq 1 ]; then
            echo "failed"
            continue
        fi
        echo "Test passed"
    done
    exit 0
}

run_small_tests() {
    echo
    echo "Running small tests (dp and bkt)"
    echo
    for test in "$argc_test_dir"/small/*.kp; do
        echo "Running $(basename "$test")"
        run_test "$test" "bkt"
        if [ $? -eq 1 ]; then
            echo "bkt failed"
        fi
        run_test "$test" "dp"
        if [ $? -eq 1 ]; then
            echo "dp failed"
            continue
        fi
        run_test "$test" "fptas"
        echo "Test passed"
    done
}

run_mid_tests() {
    echo
    echo "Running mid tests (dp only)"
    echo

    for test in "$argc_test_dir"/mid/*.kp; do
        echo "Running $(basename "$test")"
        run_test "$test" "dp"
        if [ $? -eq 1 ]; then
            echo "dp failed"
            continue
        fi
        run_test "$test" "fptas"
        echo "Test passed"
    done
}

run_large_tests() {
    echo
    echo "Running large tests (dp only)"
    echo

    for test in "$argc_test_dir"/large/*.kp; do
        echo "Running $(basename "$test")"
        run_test "$test" "dp"
        if [ $? -eq 1 ]; then
            echo "dp failed"
            continue
        fi
        run_test "$test" "fptas"
        echo "Test passed"
    done
}

init() {
    xmake f -m release
    xmake clean
    xmake

    cargo build -r 2>/dev/null

    # Save the output if specified
    if [ "$argc_save" ]; then
        mkdir -p $argc_output_dir/{bkt,dp,fptas}
    fi
}

# @cmd Run a single test
# @arg test! <PATH>   The path to the test file
# @arg method! <METHOD>     The method/algorithm to be used
run_one_test() {
    # Recalculate the answer
    init
    # Precalculate the answer
    xmake r knapsack_dp <"$argc_test" >$argc_answers_dir/$(basename "$argc_test" .kp).ans

    run_test "$argc_test" "$argc_method"
    if [ $? -eq 1 ]; then
        echo "Test $argc_test failed"
    else
        echo "Test $argc_test passed"
    fi
    exit 0
}

# @cmd Run all tests
# @flag --special Include special tests
run_all_tests() {
    init

    # Precalculate the answers
    if [ ! -d $argc_answers_dir ]; then
        echo "Precalculating answers"
        mkdir -p $argc_answers_dir
        find "$argc_test_dir" -name "*.kp" -exec bash -c 'xmake r knapsack_dp <$0 >$1/$(basename "$0" .kp).ans' {} $argc_answers_dir \;
    fi

    run_small_tests
    run_mid_tests
    run_large_tests

    if [ "$argc_special" ]; then
        run_special_tests
    fi

    exit 0
}

eval "$(argc --argc-eval "$0" "$@")"
