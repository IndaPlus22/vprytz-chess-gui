# vprytz-chess-gui

A bad chess GUI, using [eskilny-task-3](https://github.com/IndaPlus22/eskilny-task-3) as chess library.

Now online only! If you want the version that does not require a server, clone the project and checkout [this commit](https://github.com/IndaPlus22/vprytz-chess-gui/commit/3fa51b5c58a6f71a75171339c7ca39ec5ff3f468).

## Controls

- Escape: exits the game immediately
- R: restarts the game immediately

## How to run

Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

Clone this project. Then use `cargo` to compile and run it.

```bash
cargo run
```

The game will ask for a server IP. Meaning, you need to setup the server first. See [vprytz-sockets](https://github.com/IndaPlus22/vprytz-sockets) for instructions.

Once connected to a server, enter a "room name". This can be anything, as long as it does not contain spaces.

On another computer (or on your computer, but in a different terminal), run the same command. This time, enter the same room name. You should now be able to play against yourself.

## How to play

It's chess. You know how to play chess, right?
