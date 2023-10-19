#!/bin/bash

# Get the current directory
CURRENT_DIR=$(pwd)

# Build docker image and run it
docker build -t filler .

# Run the container in detached mode
docker run -v "$(pwd)/solution":/filler/solution -it filler /bin/bash

# Attach to the container interactively
# docker -it filler_container /bin/bash