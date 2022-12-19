This is part of a series of fun little programs I write to learn rust:

- [Alder](https://github.com/coijanovic/alder): Conway's Game of Life
- [Balsa](https://github.com/coijanovic/balsa): Image to Emoji

# Alder

[Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life) has been implemented a million times, so what's one more?

# Running

To run, use `cargo run .` in the repo's root folder.

# Configuration

The *game* can be configured via three contants in [src/main.rs](./src/main.rs):

- `SIZE` determines the size of the (square) playing field in characters.
  Choose a number that is at most equal to the minimum of your terminal's width and height.
- `CHANCE` determines how likely a node is to be alive in the inital configuration.
- `SLEEP_INTERVAL` determines how fast the game runs.
  The lower the number, the more often the field is updated.
