#!/bin/bash

# Source folder path (replace with your actual path)
source_folder="/media/awais/Data3/UraanAI/projects/Cerberus-Wallet/cerberus-firmware/legacy/firmware/protob"

# Target folder path (replace with the new location)
target_folder="../../vendor/cerberus-common/protob"

# Loop through all *.proto files
for file in "$source_folder"/*.proto; do
  # Extract filename without extension
  filename="${file##*/}"

  # Construct the symlink path
  symlink_path="$source_folder/$filename"

  # Update the symlink (forcefully if needed)
  ln -sf "$target_folder/$filename" "$symlink_path"
done

