#!/bin/bash

if [ "$#" -ne 3 ]; then
    echo "Illegal number of parameters"
    echo "Usage: $0 <inputs> <outputs> <verifier_executable>"
    exit 1
fi


IN_DIR=$1
OUT_DIR=$2
VERIFIER=$3

readarray -d '' instances < <(find "$IN_DIR" -maxdepth 1 -type f -print0 | sort -z)
readarray -d '' solutions < <(find "$OUT_DIR" -maxdepth 1 -type f -print0 | sort -z)

echo Running dataset: $DATASET_DIR

for (( i=0; i<${#instances[@]}; i++ )); do
    # Get the file from each array using the index 'i'
    instance="${instances[i]}"
    solution="${solutions[i]}"

    echo "-------:-------"
    echo "Verifying input file: $instance"
    echo "Verifying solution file: $solution"
    ./$VERIFIER $instance
    $VERIFIER $solution $instance
done