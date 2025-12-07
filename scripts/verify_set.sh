#!/bin/bash

set -e

if [ "$#" -ne 2 ]; then
    echo "Illegal number of parameters"
    echo "Usage: $0 <dataset_directory> <verifier_executable>"
    exit 1
fi


DATASET_DIR=$1
VERIFIER=$2


for dir in ${DATASET_DIR}/*; do
    IFS='/' read -ra ADDR <<< "$dir"
    DATA_INDEX=${ADDR[-1]}
    if [ -d "$dir/in" ]; then
        mv "$dir/in" "$dir/IN"
    fi
    if [ -d "$dir/out" ]; then
        mv "$dir/out" "$dir/OUT"
    fi

    IN_DIR=${dir}/IN
    OUT_DIR=${dir}/OUT

    readarray -d '' instances < <(find "$IN_DIR" -maxdepth 1 -type f -print0 | sort -z)
    readarray -d '' solutions < <(find "$OUT_DIR" -maxdepth 1 -type f -print0 | sort -z)

    echo Running dataset: $dir

    for (( i=0; i<${#instances[@]}; i++ )); do
        # Get the file from each array using the index 'i'
        instance="${instances[i]}"
        solution="${solutions[i]}"

        echo "-------:-------"
        echo "Verifying input file: $instance"
        ./$VERIFIER --instance-file $instance
        ./$VERIFIER --instance-file $instance $solution 
    done
    echo "-------:-------"
done