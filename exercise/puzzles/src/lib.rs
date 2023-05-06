use log::{error, info};
use std::fs::File;
use thiserror::Error;

pub const PUZZLE_PIECES: u32 = 42;

/// This is a Piece!
#[derive(Debug, Clone)]
pub struct Puzzle {
    /// Number of piece
    pub num_pieces: u32,
    /// Descriptive name
    pub name: String,
}

impl Puzzle {
    /// Make a new puzzle!
    pub fn new() -> Self {
        let puzzle = Default::default();
        info!("Created a puzzle with new(): {:?}", puzzle);
        puzzle
    }

    /// Load a puzzle from a file
    pub fn from_file(_fh: File) -> Result<Self, PuzzleError> {
        error!("This file is missing a piece!");
        Err(PuzzleError::MissingPiece)
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Puzzle {
            num_pieces: PUZZLE_PIECES,
            name: "Default".to_string(),
        }
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PuzzleError {
    #[error("Missing a piece")]
    MissingPiece,
    #[error("Piece {0} doesn't fit!")]
    WontFit(u16),
}
