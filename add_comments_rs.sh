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
      # If Rust file
      local rel_path=${file#"$start_dir"/}  # Get relative path
      
      # Remove any existing // File: comments
      sed -i '' '/^\/\/ File: /d' "$file"
      
      # Prepend the correct // File: comment
      awk -v comment="// File: $rel_path" '
        BEGIN {print comment}
        {print}
      ' "$file" > "${file}.tmp" && mv "${file}.tmp" "$file"
    fi
  done
}

# Run function from starting directory
process_files "$start_dir"
