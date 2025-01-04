#!/bin/bash

# Usage: ./check.sh [--clear] [--save] [--help]
# --clear: Clear the output directory
# --save: Save the output of the checker
# --help: Display this message

if [ "$1" == "--help" ]; then
    echo "Usage: ./check.sh [--clear] [--save] [--help]"
    echo "--clear: Clear the output directory"
    echo "--save: Save the output of the checker"
    echo "--help: Display this message"
    exit 0
fi

# Clear the output directory

if [ "$1" == "--clear" ]; then
    rm -rf ../output
    exit 0
fi

TEST_DIR=./tests
ANSWERS_DIR=./answers
OUTPUT_DIR=./output

xmake f -m release
xmake clean
xmake

cargo build -r 2>/dev/null

rm debug.log

# Precalculate the answers
if [ ! -d $ANSWERS_DIR ]; then
    mkdir -p $ANSWERS_DIR
    find "$TEST_DIR" -name "*.kp" -exec sh -c 'xmake r knapsack_dp <"$1" >$ANSWERS_DIR/$(basename "$0" .kp).ans' {} \;
fi

# Save the output if specified
if [ "$1" == "--save" ]; then
    mkdir -p $OUTPUT_DIR/{bkt,dp,fptas}
fi

# Run the tests
# $1: test file
# $2: algorithm
run_test() {
    ANSWER=$(cat $ANSWERS_DIR/$(basename "$1" .kp).ans)
    cargo run -r -- --input-file "$1" run "$2" 2>>debug.log
    # Output saved in out.json
    if [ "$3" == "--save" ]; then
        cp out.json $OUTPUT_DIR/"$2"/$(basename "$1" .kp).json
    fi
    VALUE=$(jq -r '.total_value' out.json)
    ITEMS=$(jq -r '.items | @sh' out.json)

    if [ "$2" == "fptas" ]; then
        # Check how close the value is to the answer
        PERCENTAGE=$(echo "scale=5; 100 * $VALUE / $ANSWER" | bc)
        echo "Fptas: VALUE is $PERCENTAGE% of ANSWER"
        return 0
    fi

    if [ "$ANSWER" != "$VALUE" ]; then
        echo "Value mismatch"
        return 1
    fi

    xmake r check $VALUE $ITEMS <"$1"
    return $?
}

echo "Running small tests (dp and bkt)"
echo

for test in "$TEST_DIR"/small/*.kp; do
    echo "Running $(basename "$test")"
    run_test "$test" "bkt" "$1"
    if [ $? -eq 1 ]; then
        echo "bkt failed"
    fi
    run_test "$test" "dp" "$1"
    if [ $? -eq 1 ]; then
        echo "dp failed"
        continue
    fi
    run_test "$test" "fptas" "$1"
    echo "Test passed"
done

echo
echo "Running mid tests (dp only)"
echo

for test in "$TEST_DIR"/mid/*.kp; do
    echo "Running $(basename "$test")"
    run_test "$test" "dp" "$1"
    if [ $? -eq 1 ]; then
        echo "dp failed"
        continue
    fi
    run_test "$test" "fptas" "$1"
    echo "Test passed"
done

echo
echo "Running large tests (dp only)"
echo

for test in "$TEST_DIR"/large/*.kp; do
    echo "Running $(basename "$test")"
    run_test "$test" "dp" "$1"
    if [ $? -eq 1 ]; then
        echo "dp failed"
        continue
    fi
    run_test "$test" "fptas" "$1"
    echo "Test passed"
done

rm out.json
