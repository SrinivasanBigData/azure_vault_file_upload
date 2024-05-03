#!/bin/bash

# Check if the correct number of arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <secret_name> <data_file>"
    exit 1
fi

# Assign arguments to variables
SECRET_NAME="$1"
DATA_FILE="$2"
IMAGE_NAME="srinivasanbigdata/azure_vault_file_upload"

# Execute the docker run command
docker run \
  -v $(pwd)/.env:/usr/src/app/.env \
  -v $(pwd)/data:/usr/src/app/data \
  $IMAGE_NAME \
  sh -c "sh azure_upload.sh --secret-name $SECRET_NAME --file-path data/$DATA_FILE"
