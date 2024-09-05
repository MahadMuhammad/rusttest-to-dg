#!/bin/bash

# Check if GCCRS_PATH is set
if [ -z "$GCCRS_PATH" ]; then
  echo -e "Error: GCCRS_PATH environment variable is not set.\nPlease set it using the command: export GCCRS_PATH=/path/to/gccrs"
  exit 1
fi

# Check if RUST_PATH is set
if [ -z "$RUST_PATH" ]; then
  echo "Error: RUST_PATH environment variable is not set."
  exit 1
fi

echo "GCCRS_PATH: $GCCRS_PATH"
echo "RUST_PATH: $RUST_PATH"

# Check if the ui directory exists and remove it if it does
if [ -d "$GCCRS_PATH/gcc/testsuite/rust/rustc/ui" ]; then
  echo "Removing existing ui directory at $GCCRS_PATH/gcc/testsuite/rust/rustc/ui"
  rm -rf "$GCCRS_PATH/gcc/testsuite/rust/rustc/ui"
fi

# Copy the RUST_PATH/tests/ui to GCCRS_PATH/gcc/testsuite/rust/rustc
echo "Copying tests from $RUST_PATH/tests/ui to $GCCRS_PATH/gcc/testsuite/rust/rustc"
cp -r "$RUST_PATH/tests/ui" "$GCCRS_PATH/gcc/testsuite/rust/rustc"
echo "Copied tests to $GCCRS_PATH/gcc/testsuite/rust/rustc"

# Function to process a single file
process_file() {
  file="$1"
  base_name="${file%.rs}"
  stderr_file="${base_name}.stderr"
  output_file="${base_name}_dg.rs"

  if [[ -f "$stderr_file" ]]; then
    rusttest-to-dg --file "$file" --stderr "$stderr_file" > "$output_file"
  else
    rusttest-to-dg --file "$file" > "$output_file"
  fi
}

export -f process_file

# Find all .rs files and process them in parallel
echo -e "\nProcessing files in $GCCRS_PATH/gcc/testsuite/rust/rustc/ui"
find "$GCCRS_PATH/gcc/testsuite/rust/rustc/ui" -name '*.rs' -print0 | xargs -0 -n 1 -P $(nproc) bash -c 'process_file "$@"' _

echo "Processing complete."