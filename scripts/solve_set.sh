#! /bin/bash

if [ "$#" -ne 3 ]; then
    echo "Illegal number of parameters"
    echo "Usage: ./solve_set.sh <dataset_directory> <output_directory> <solver_executable>"
    exit 1
fi

INDEX=155927

DATASET_DIR=$1
OUTPUT_DIR=$2
SOLVER=$3

tm_millisecs() {
    local start=${EPOCHREALTIME/./}
    "$@"
    local exit_code=$?
    echo "$(( (${EPOCHREALTIME/./} - start)/1000 ))"
    return ${exit_code}
}

# echo Running dataset: $DATASET_DIR

# for dir in ${DATASET_DIR}/*; do
#     IFS='/' read -ra ADDR <<< "$dir"
#     DATA_INDEX=${ADDR[-1]}
#     echo "Solving dataset index: $DATA_INDEX"
#     for instance in ${DATASET_DIR}/${DATA_INDEX}/IN/*; do
#         IFS='/' read -ra ADDR2 <<< "$instance"
#         input=${ADDR2[-1]}
#         IFS='_' read -ra ADDR3 <<< "${input:0:-4}"
#         instance_size=${ADDR3[-1]}
#         echo "  Solving size: $instance_size" 
#         input_file=${DATASET_DIR}/${DATA_INDEX}/IN/${input}
#         output_file=${OUTPUT_DIR}/out_${INDEX}_${DATA_INDEX}_${instance_size}.txt
#         echo "    Input file: $input_file"
#         echo "    Output file: $output_file"
#         tm_millisecs $SOLVER $input_file $output_file 2>&1 >> ${OUTPUT_DIR}/out_${INDEX}_${DATA_INDEX}_${instance_size}_time.txt
#         echo "    Done."
#     done
# done

echo "Creating paste file..."

INDEX_ORDER=( 155925 155878 155827 155855 155904 155935 155942 155927 150252 155997 155859 155915 )
SIZE_ORDER=( 50 100 150 200 250 300 350 400 450 500 )

paste_file=${OUTPUT_DIR}/paste_${INDEX}_f.txt
time_paste_file=${OUTPUT_DIR}/time_paste_${INDEX}_f.txt

for index in "${INDEX_ORDER[@]}"; do
    for size in "${SIZE_ORDER[@]}"; do
        output_file=${OUTPUT_DIR}/out_${INDEX}_${index}_${size}.txt
        time_output_file=${OUTPUT_DIR}/out_${INDEX}_${index}_${size}_time.txt
        echo "  Processing output file: $output_file"
        score=$(head -n 1 $output_file | cut -d' ' -f2) || echo ""
        echo "    Score: $score"
        echo "$score" >> $paste_file
        echo "$(cat $time_output_file)" >> $time_paste_file
    done
    echo "  Paste file created at: $paste_file"
done
