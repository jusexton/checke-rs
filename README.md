# checke-rs

![](https://github.com/jsextonn/checke-rs/workflows/build/badge.svg)

Feature rich and easy to use checkers engine written in the rust programming language.

## Quick Look

```rust
fn main() {
    // Initializes a new board with classical checkers starting positions.
    let board = Board::default();

    // Commence the game by pushing turns. 
    // Turns can be represented by many different types and can consist of many moves.
    board.push_turn("23x18").unwrap();

    // Easily revert turns by popping the last turn made.
    let popped_state = board.pop_turn();

    // The state of the game can easily be iterated via history.
    for state in board.history {
        // Iterate each state the board was ever in
    }

    // Convenient methods for retrieving commonly accessed state.
    let initial_state = board.initial_state();
    let current_state = board.current_state();

    // Board state contains the state of the game at a given point in time.
    let black_pieces = current_state.black_pieces();
    let red_kings = current_state.red_kings();
}
```

## Warning

This library is still under heavy development and breaking changes to the API are almost a certainty!

## License

This project is licensed under the [MIT license].

[mit license]: https://github.com/JSextonn/checke-rs/blob/master/LICENSE