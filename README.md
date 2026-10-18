# Filler

A robot that plays **Filler**: two robots take turns placing shapes on a grid,
and whoever covers the most squares wins.

Our robot is written in Rust and lives in the `solution` folder.

## The rules

Each turn the robot gets the board and a shape. The shape must:

1. cover **exactly one** of our squares
2. cover **none** of the enemy's squares
3. stay **inside** the board

The robot answers with the position as `X Y` (column, then row).

## How it chooses a move

1. Find every allowed spot.
2. Keep the 10 spots closest to the enemy.
3. For each one, count the empty squares we would reach before the enemy.
4. Pick the spot that wins the most squares.

## Files

| File | What it does |
|---|---|
| `solution/src/main.rs` | reads the game, asks for a move, prints the answer |
| `solution/src/parser.rs` | turns the game's text into a board and a shape |
| `solution/src/placement.rs` | checks the 3 rules and lists all allowed spots |
| `solution/src/strategy.rs` | picks the best spot |
| `solution/src/output.rs` | writes the answer as `X Y` |
| `solution/src/bin/visualizer.rs` | shows a game in color |
| `solution/src/tests/` | the unit tests |

## Filler docker image

- To build the image `docker build -t filler .`
- To run the container `docker run -v "$(pwd)/solution":/filler/solution -it filler`. This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.
- Example of a command in the container `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator`
- Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

On Windows PowerShell, use `${PWD}` instead of `$(pwd)`.

### Notes

- `Terminator` is a very strong robot so it's optional to beat him.
- For M1 Macs use `m1_robots` and `m1_game_engine`.

## Run it

Inside the container:

```bash
cd solution
cargo build --release
cargo test
cd ..
```

Play a game:

```bash
./linux_game_engine -f maps/map00 -p1 solution/target/release/filler -p2 linux_robots/wall_e
```

Watch it in color (add a number at the end for the speed, in milliseconds):

```bash
./linux_game_engine -f maps/map01 -p1 solution/target/release/filler -p2 linux_robots/h2_d2 | solution/target/release/visualizer
```

## Results

| Map | Enemy | Wins |
|---|---|---|
| map00 | wall_e | 5 / 5 |
| map01 | h2_d2 | 5 / 5 |
| map02 | bender | 5 / 5 |
| map02 | terminator | 10 / 10 |

Use **map02** for the terminator bonus.
