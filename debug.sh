#!/bin/bash

if [ "$#" -ne 3 ]; then
    echo "Illegal number of parameters"
    echo "Usage: $0 <dataset_directory> <output_directory> <executable>"
    exit 1
fi

DATASET_DIR=$1
OUTPUT_DIR=$2
EXECUTABLE=$3


for dir in ${DATASET_DIR}/*; do
  IN_DIR=$dir/IN

  echo Running dataset: $dir
  IFS='/' read -ra ADDR <<< "$dir"
  DATA_INDEX=${ADDR[-1]}
  for instance in "$IN_DIR"/*; do
      IFS='/' read -ra ADDR2 <<< "$instance"
      input=${ADDR2[-1]}
      IFS='_' read -ra ADDR3 <<< "${input:0:-4}"
      instance_size=${ADDR3[-1]}
      echo "  Solving size: $instance_size" 
      input_file=${DATASET_DIR}/${DATA_INDEX}/IN/${input}
      output_file=${OUTPUT_DIR}/out_${INDEX}_${DATA_INDEX}_${instance_size}.txt
      echo "    Input file: $input_file"
      echo "    Output file: $output_file"
      echo "-------:-------"
      echo "Debugging input file: $instance"
      ./$EXECUTABLE $input_file . > $output_file
  done
done


