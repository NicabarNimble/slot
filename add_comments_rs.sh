#!/bin/bash

# Starting directory
start_dir="."

# Function to process files
process_files() {
  local dir=$1
  for file in "$dir"/*; do
    if [ -d "$file" ]; then
      # Skip the target directory
      if [[ $file == "$dir/target" ]]; then
        continue
      fi
      # If directory, recurse
      process_files "$file"
    elif [[ $file == *.rs ]]; then
      # If Rust file, check for existing comment and prepend if not found
      local rel_path=${file#"$start_dir"/}  # Get relative path
      if ! grep -q "// File: $rel_path" "$file"; then
        echo "// File: $rel_path" | cat - "$file" > temp && mv temp "$file"
      fi
    fi
  done
}

# Run function from starting directory
process_files "$start_dir"
