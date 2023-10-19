#!/bin/bash

# Clone the repository from GitHub
git clone https://github.com/bomanviktor/filler.git

# Navigate into the cloned repository directory
cd filler

# Ensure that build.sh is executable
chmod +x build.sh

# Build and run the container, keeping it alive with an endless loop
./build.sh
# Execute the compile script within the running container
#docker exec -it filler_container ./compile.sh

# Attach to the container interactively
#docker exec -it filler_container /bin/bash