I'm trying to learn Rust, so I've implemented a minimal version of [https://en.wikipedia.org/wiki/Conway's_Game_of_Life](Conway's Game of Life).

# Running

To run, use `cargo run .` in the repo's root folder.

# Configuration

The *game* can be configured via three contants in [src/main.rs](`src/main.rs`):

- `SIZE` determines the size of the (square) playing field in characters.
  Choose a number that is at most equal to the minimum of your terminal's width and height.
- `CHANCE` determines how likely a node is to be alive in the inital configuration.
- `SLEEP_INTERVAL` determines how fast the game runs.
  The lower the number, the more often the field is updated.
