#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Usage: $0 folder_path"
  exit 1
fi

# get the input folder path
folder="$1"

# check if the input folder exists
if [ ! -d "$folder" ]; then
  echo "Directory not found: $folder"
  exit 1
fi

# get the absolute path of the input folder
folder="$(
  cd "$(dirname "$folder")"
  pwd
)/$(basename "$folder")"

# List folders to ignore
declare -a ignore_folders=(".git" "node_modules" "build" "dist" "temp" "assets" "target" "pkg")

# List files to ignore
declare -a ignore_files=(
  "package-lock.json"
  "yarn.lock"
  "log.txt"
  "readme.md"
  "README.md"
)

# Build the find command
cmd=(find "$folder" -type f)
for ignore_folder in "${ignore_folders[@]}"; do
  cmd=("${cmd[@]}" -not -path "*/$ignore_folder/*")
done

# Add ignore files
for ignore_file in "${ignore_files[@]}"; do
  cmd=("${cmd[@]}" -not -name "$ignore_file")
done

cmd=("${cmd[@]}" -not -path '.*' -print0) # Exclude hidden files

# Traverse the folder and concatenate files
"${cmd[@]}" | while IFS= read -r -d '' file; do
  # Check if the file is a text file
  file_type=$(file --brief --mime-type "$file")
  if [[ "$file_type" = "text/plain" ]]; then
    # Get the relative path to the file with respect to the input folder
    relative="${file#$folder}"

    # print the relative path followed by the file contents
    echo "===$relative==="
    cat "$file"
    echo ""
  fi
done
