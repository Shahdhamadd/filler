# Filler

This is my robot for **Filler**, a little game where two robots take turns
dropping shapes onto a grid. Whoever ends up covering more of the board wins.

I wrote the robot in Rust. Everything I built lives in the `solution` folder.
The rest of this repo (the game engine, the maps and the enemy robots) came
with the project.

## How the game works

Every turn the game engine sends my robot two things: the current board and a
new shape to place. A placement only counts if the shape:

1. touches **exactly one** square I already own
2. doesn't touch **any** of the enemy's squares
3. stays **inside** the board

The robot replies with where it wants to put the shape, written as `X Y`
(column first, then row). If it can't find a legal spot, the game is over for
it.

## How my robot thinks

Just grabbing the spot closest to the enemy is enough against the weaker
robots, but the strong ones punish it because it ignores what happens next. So
my robot looks one step further:

1. Lists every legal spot for the shape.
2. Keeps only the 10 spots that are closest to the enemy, so it stays
   aggressive and still runs fast.
3. For each of those, imagines the board after the move and counts how many
   empty squares I could reach before the enemy can.
4. Plays the spot that gives me the most territory.

The idea is to push toward the enemy and close off space, so they run out of
room before I do.

## What's in the code

| File | What it does |
|---|---|
| `solution/src/main.rs` | the main loop: read the turn, pick a move, answer |
| `solution/src/parser.rs` | turns the engine's text into a board and a shape |
| `solution/src/placement.rs` | checks the three rules and finds every legal spot |
| `solution/src/strategy.rs` | decides which spot is best |
| `solution/src/output.rs` | prints the answer as `X Y` |
| `solution/src/bin/visualizer.rs` | replays a game in color so you can watch it |
| `solution/src/tests/` | unit tests |

## Running it

Everything runs inside Docker, so you don't need Rust installed on your own
machine.

Build the image:

```bash
docker build -t filler .
```

Start the container. This mounts the `solution` folder inside it, so any change
you make on your machine shows up in the container straight away:

```bash
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

On Windows PowerShell, use `${PWD}` instead of `$(pwd)`.

Once you're inside, build the robot and run the tests:

```bash
cd solution
cargo build --release
cargo test
cd ..
```

Then play a game against one of the provided robots:

```bash
./linux_game_engine -f maps/map00 -p1 solution/target/release/filler -p2 linux_robots/wall_e
```

If you want to actually watch the game, pipe it into the visualizer. You can
add a number at the end to set the speed in milliseconds:

```bash
./linux_game_engine -f maps/map01 -p1 solution/target/release/filler -p2 linux_robots/h2_d2 | solution/target/release/visualizer
```

On an M1 Mac, use `m1_game_engine` and `m1_robots` instead of the `linux_` ones.

## How it did

I played each matchup several times to make sure the wins weren't luck:

| Map | Enemy | Wins |
|---|---|---|
| map00 | wall_e | 5 / 5 |
| map01 | h2_d2 | 5 / 5 |
| map02 | bender | 5 / 5 |
| map02 | terminator | 10 / 10 |

Terminator is the strongest robot, and beating it is an optional bonus. My
robot beats it every time on **map02**, so use that map if you want to see it.
