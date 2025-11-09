#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Illegal number of parameters"
    echo "Usage: $0 <dataset_directory> <verifier_executable>"
    exit 1
fi


DATASET_DIR=$1
VERIFIER=$2

IN_DIR=${DATASET_DIR}/in
OUT_DIR=${DATASET_DIR}/out

readarray -d '' instances < <(find "$IN_DIR" -maxdepth 1 -type f -print0 | sort -z)
readarray -d '' solutions < <(find "$OUT_DIR" -maxdepth 1 -type f -print0 | sort -z)

echo Running dataset: $DATASET_DIR

for (( i=0; i<${#instances[@]}; i++ )); do
    # Get the file from each array using the index 'i'
    instance="${instances[i]}"
    solution="${solutions[i]}"

    echo "-------:-------"
    echo "Verifying input file: $instance"
    ./$VERIFIER $instance
    $VERIFIER $solution $instance
done