//! # Connect4
//!
//! `connect4` is a library to handle the logic of the board game of the same name.

use std::fmt::{self, Display};

use gamesweet::Game;
use itertools::Itertools;

/// Size of the game board.
const ROWS: usize = 6;
const COLS: usize = 7;

/// Connect4 game.
#[derive(Clone, Debug)]
pub struct Connect4 {
    board: Board,
    player: Player,
}

impl Connect4 {
    /// Create a new Connect4 game.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Connect4 {
    fn default() -> Self {
        Self {
            board: Board::new(),
            player: Player::Black,
        }
    }
}

impl Display for Connect4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}

impl Game for Connect4 {
    type Player = Player;
    type Turn = Turn;

    /// Get the current player.
    fn player(&self) -> Player {
        self.player
    }

    /// Get all legal turns.
    fn turns(&self) -> Vec<Turn> {
        self.board.turns(self.player)
    }

    /// Play a turn of the game.
    fn play(&mut self, turn: Turn) -> bool {
        if turn.player != self.player {
            return false;
        }

        let played = self.board.play(&turn);
        self.player.switch();

        played
    }

    /// Check if the game is over.
    fn over(&self) -> bool {
        self.board.over()
    }

    /// Get the winner of the game.
    ///
    /// Returns `None` if the game is still ongoing.
    fn winner(&self) -> Option<Player> {
        self.board.winner()
    }
}

/// Board on which the game is played.
///
/// Responsible for managing the placement of pieces and handling game logic.
#[derive(Clone, Debug, PartialEq)]
struct Board([[Square; COLS]; ROWS]);

impl Board {
    /// Create a new Board.
    ///
    /// The board starts with 4 pieces in the centre.
    /// The first player is always black.
    fn new() -> Self {
        Self([[Square::Empty; COLS]; ROWS])
    }

    /// Get all legal turns for the current player.
    fn turns(&self, player: Player) -> Vec<Turn> {
        let mut turns = Vec::new();

        // Iterate through the entire board
        for col in (0..COLS)
            .into_iter()
            .filter(|col| !self.0[ROWS - 1][*col].taken())
        {
            turns.push(match Turn::new(player, col) {
                Some(turn) => turn,
                None => unreachable!(),
            });
        }

        turns
    }

    /// Play a turn of the game.
    fn play(&mut self, turn: &Turn) -> bool {
        for row in self.0.iter_mut() {
            let square = &mut row[turn.pos];
            if !square.taken() {
                *square = Square::Piece(turn.player);
                return true;
            }
        }

        false
    }

    /// Check if the game is over.
    fn over(&self) -> bool {
        self.0[ROWS - 1].iter().all(|s| s.taken()) || self.winner().is_some()
    }

    /// Get the winner of the game.
    ///
    /// Returns `None` if the game is still ongoing.
    fn winner(&self) -> Option<Player> {
        // Declare a closure to check for a win in a line
        let connect4 = |v: &[&Square]| -> Option<Player> {
            const CONNECT: usize = 4;

            for four in v.windows(CONNECT) {
                if four.iter().unique().count() == 1 {
                    match four.last() {
                        Some(Square::Piece(player)) => return Some(*player),
                        _ => continue,
                    };
                }
            }

            None
        };

        // Create a vec of lines to check
        let mut lines = Vec::<Vec<&Square>>::new();

        // Add all rows
        for row in self.0.iter() {
            lines.push(row.iter().collect());
        }
        // Add all cols
        for col in 0..ROWS {
            lines.push(self.0.iter().map(|row| &row[col]).collect());
        }
        // Add all diagonals
        for i in 0..ROWS {
            lines.push(
                (0..i)
                    .rev()
                    .zip(0..COLS)
                    .map(|(row, col)| &self.0[row][col])
                    .collect(),
            );
            lines.push(
                (i..ROWS)
                    .zip(0..COLS)
                    .map(|(row, col)| &self.0[row][col])
                    .collect(),
            );
        }
        for i in 1..COLS {
            lines.push(
                (i..COLS)
                    .zip(0..ROWS)
                    .map(|(col, row)| &self.0[row][col])
                    .collect(),
            );
            lines.push(
                (i..COLS)
                    .rev()
                    .zip(0..ROWS)
                    .map(|(col, row)| &self.0[row][col])
                    .collect(),
            );
        }

        // Check all lines for a win
        for it in lines.into_iter().filter(|it| it.len() >= 4) {
            let win = connect4(&it);
            if win.is_some() {
                return win;
            }
        }

        None
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print top border
        writeln!(f, "┌{}─┐", "─".repeat(2 * COLS))?;

        // Print row labels
        write!(f, "│")?;
        for i in 0..COLS {
            write!(f, " {}", i + 1)?;
        }
        writeln!(f, " │")?;
        writeln!(f, "├{}─┤", "─".repeat(2 * COLS))?;

        // Print each row of the board
        for row in self.0.iter().rev() {
            write!(f, "│")?;
            for square in row.iter() {
                write!(f, " {}", square)?;
            }
            writeln!(f, " │")?;
        }

        // Print bottom border
        write!(f, "└{}─┘", "─".repeat(2 * COLS))
    }
}

/// A square of the game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Square {
    Piece(Player),
    Empty,
}

impl Square {
    /// Check if a square is taken.
    fn taken(&self) -> bool {
        match self {
            Square::Piece(_) => true,
            Square::Empty => false,
        }
    }
}

impl Display for Square {
    /// Display a game square.
    ///
    /// | Piece                   | Char |
    /// | ----------------------- | ---- |
    /// | `Piece(Player::Black)`  | `●`  |
    /// | `Piece(Player::White)`  | `○`  |
    /// | `Empty`                 | `_`  |
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Piece(player) => write!(f, "{}", player),
            Square::Empty => write!(f, "_"),
        }
    }
}

/// A player of the game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Player {
    Black,
    White,
}

impl Player {
    /// Get the opponent of a player.
    pub fn opponent(&self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    /// Switch player to opponent.
    fn switch(&mut self) {
        *self = self.opponent();
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::Black => "●",
                Player::White => "○",
            }
        )
    }
}

/// A board position to play a piece.
#[derive(Clone, Debug, PartialEq)]
pub struct Turn {
    player: Player,
    pos: usize,
}

impl Turn {
    /// Create a new Turn.
    pub fn new(player: Player, pos: usize) -> Option<Self> {
        match pos {
            pos if pos < COLS => Some(Self { player, pos }),
            _ => None,
        }
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pos + 1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
