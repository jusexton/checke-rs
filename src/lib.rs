//! # Checke-rs
//! A checkers engine exposing features around managing board state in the cleanest way possible.

pub mod board;
pub mod bitboard;
pub mod turn;
pub mod position;

#[macro_use]
extern crate num_derive;
