# checke-rs

![](https://github.com/jsextonn/checke-rs/workflows/build/badge.svg)

Feature rich and easy to use checkers engine written in the rust programming language.

## Quick Look

```rust
fn main() {
    // Initializes a new board with classical checkers starting positions.
    let board = Board::default();
    
    // Turns can be made using various different types.
    board.push_turn("23x18").unwrap();
    
    // Boards contain a stack representing all successful turns. 
    // This allows you to easily traverse the history of the game.
    let initial_state = board.state_stack.front();
    
    // The state stack even allows for easy turn reverting
    board.pop_turn();
    
    // Boards current state can be easily retrieved and read.
    let current_state = board.current_state();
    
    // Board state contains the current player and piece data in the form of bitboards.
    let red_kings = current_state.red_kings();
}
```

## Warning

This library is still under heavy development and breaking changes to the API are almost a certainty!

## License

This project is licensed under the [MIT license].

[mit license]: https://github.com/JSextonn/checke-rs/blob/master/LICENSE