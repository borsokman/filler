# Filler docker image

- To build the image `docker build -t filler .`
- To run the container `docker run --rm -v "$(pwd)/solution":/filler/solution -it filler`. This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.
- Example of a command in the container `./m1_game_engine -f maps/map00 -p1 m1_robots/wall_e -p2 m1_robots/h2_d2 > /filler/solution/logs/game_log.txt 2>&1`
- Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

## Notes

- `Terminator` is a very strong robot so it's optional to beat him.
- For M1 Macs use `m1_robots` and `m1_game_engine`.

Recompile steps:

docker run --rm -v "$(pwd)/solution":/filler/solution -it filler
cd /filler/solution
cargo build --release --bin bot
cp target/release/solution bot
cd /filler

Vs h2_d2

./m1_game_engine -f maps/map00 -p1 solution/bot -p2 m1_robots/h2_d2 > solution/logs/h2_d2_map00.txt 2>&1
./m1_game_engine -f maps/map01 -p1 solution/bot -p2 m1_robots/h2_d2 > solution/logs/h2_d2_map01.txt 2>&1
./m1_game_engine -f maps/map02 -p1 solution/bot -p2 m1_robots/h2_d2 > solution/logs/h2_d2_map02.txt 2>&1

Vs wall_e

./m1_game_engine -f maps/map00 -p1 solution/bot -p2 m1_robots/wall_e > solution/logs/wall_e_map00.txt 2>&1
./m1_game_engine -f maps/map01 -p1 solution/bot -p2 m1_robots/wall_e > solution/logs/wall_e_map01.txt 2>&1
./m1_game_engine -f maps/map02 -p1 solution/bot -p2 m1_robots/wall_e > solution/logs/wall_e_map02.txt 2>&1

Vs bender

./m1_game_engine -f maps/map00 -p1 solution/bot -p2 m1_robots/bender > solution/logs/bender_map00.txt 2>&1
./m1_game_engine -f maps/map01 -p1 solution/bot -p2 m1_robots/bender > solution/logs/bender_map01.txt 2>&1
./m1_game_engine -f maps/map02 -p1 solution/bot -p2 m1_robots/bender > solution/logs/bender_map02.txt 2>&1

Vs terminator

./m1_game_engine -f maps/map00 -p1 solution/bot -p2 m1_robots/terminator > solution/logs/terminator_map00.txt 2>&1
./m1_game_engine -f maps/map01 -p1 solution/bot -p2 m1_robots/terminator > solution/logs/terminator_map01.txt 2>&1
./m1_game_engine -f maps/map02 -p1 solution/bot -p2 m1_robots/terminator > solution/logs/terminator_map02.txt 2>&1

Visualizer inside /solution:

./target/release/visualizer logs/h2_d2_map00.txt

./target/release/visualizer logs/wall_e_map00.txt

./target/release/visualizer logs/bender_map00.txt

./target/release/visualizer logs/terminator_map00.txt

export LIBRARY_PATH="/opt/homebrew/lib"
export CPLUS_INCLUDE_PATH="/opt/homebrew/include"
export CPATH="/opt/homebrew/include"
export PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig"
