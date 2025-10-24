#! /bin/bash

INDEX=155927

SET_PATH=$1

echo $SET_PATH

for file in $SET_PATH; do
    echo "Processing file: $file"
    # Check if the item is a regular file (optional but recommended)
    if [ -f "$file" ]; then
        echo "input/$SET_PATH/$file output/$SET_PATH/out_$INDEX_${SET_PATH:3}"
        cargo run --bin solver input/$SET_PATH/$file output/$SET_PATH/out_$INDEX_${SET_PATH:3}
    fi
done