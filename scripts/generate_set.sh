#! /bin/bash

if [ "$#" -ne 2 ]; then
    echo "Illegal number of parameters"
    echo "Usage: ./generate_set.sh <output_directory> <generator_executable>"
    exit 1
fi

INDEX=155927

DATASET_DIR=$1
GENERATOR=$2

echo Generating instances at: $DATASET_DIR

SIZES=( 50 100 150 200 250 300 350 400 450 500 )

# Optional
SEEDS=( 0 1 2 3 4 5 6 7 8 9 )

if [ -z ${USE_SEEDS+x} ]; then
  echo "Using seeds for generation"
fi

for i in {0..9}; do
    SIZE=${SIZES[$i]}
    if [ "$USE_SEED" = true ]; then
        SEED=${SEEDS[$i]}
        echo "Using seed: $SEED"
    fi
    OUTPUT_FILE=${DATASET_DIR}/in_${INDEX}_${SIZE}.txt
    echo "  Generating size: $SIZE"
    echo "    Output file: $OUTPUT_FILE"
    $GENERATOR --size $SIZE --output-dir $OUTPUT_FILE $SEED
done