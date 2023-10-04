# Filler docker image
- To build the image `docker build --platform=linux/amd64 -t filler .`
- To run the container `docker run -v "$(pwd)/solution":/filler/solution -it filler`. This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.
- Example of a command in the container `./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator`
- Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

## Notes
- `Terminator` is a very strong robot so it's optional to beat him.
