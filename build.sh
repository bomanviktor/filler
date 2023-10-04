cargo build --target-dir docker_image/solution
cd docker_image || exit
docker build -t filler .
docker run -v "$(pwd)/solution":/filler/solution -it filler
