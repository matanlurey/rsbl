# rsbl

A Rust implementation of the game [Battle Line][].

[battle line]: https://boardgamegeek.com/boardgame/760/battle-line

## Running

Currently there is only the simplest prototype of the game:

- a text-based table with ⚑ characters representing flags
- colored square values representing cards (`0` is `10`)

```sh
$ cargo run

    2
    1
⚑   ⚑   ⚑   ⚑   ⚑   ⚑   ⚑
                3       0

```
