#!/bin/sh

# Add missing import
export LD_LIBRARY_PATH="../target/release:../target/debug"
# MacOS
export DYLD_LIBRARY_PATH="../target/release:../target/debug"

# Compile the C test file
gcc test_print_pretty.c -L ../target/release -L ../target/debug -lbat_c -o test_print_pretty

# Initialize test result
ALL_TESTS_PASSED=true

# Run the test and capture the output
OUTPUT=$(./test_print_pretty)
EXPECTED_OUTPUT=$(cat expected_output.txt)

echo "Testing print_pretty function"
echo "Expected: $EXPECTED_OUTPUT"
echo "     Got: $OUTPUT"

if [ "$OUTPUT" = "$EXPECTED_OUTPUT" ]; then
  echo "Test passed successfully for print_pretty"
else
  echo "Test failed for print_pretty :("
  ALL_TESTS_PASSED=false
fi

# Final test result
if [ "$ALL_TESTS_PASSED" = true ]; then
  echo "All tests passed successfully"
  exit 0
else
  echo "Some tests failed :("
  exit 1
fi
