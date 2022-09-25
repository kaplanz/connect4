use std::io::{self, Write};

use connect4::{Connect4, Player, Turn};
use gamesweet::{ai, Config, Game, TurnFn};

fn main() {
    // Create a Connect4 game
    let game = Connect4::new();

    // Define the game config
    let p1 = (Player::Black, ask_human as TurnFn<Connect4>);
    let p2 = (Player::White, ai::mcts::run as TurnFn<Connect4>);
    let config = Config::new(p1, p2);

    // Run the game loop
    game.main(config);
}

fn ask_human(game: &Connect4) -> Turn {
    // Print available turns
    println!("Available turns:");
    for turn in game.turns() {
        println!("{}", turn);
    }

    // Query the game for the player
    let player = Game::player(game);

    // Loop until user provides a valid turn
    loop {
        // Print prompt
        print!("[{}] >> ", &player);
        io::stdout().flush().unwrap();

        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Process input
        let input = input.trim().as_bytes();
        if input.is_empty() {
            continue;
        }

        // Validate input
        if input.len() != 1 {
            eprintln!("error: invalid input");
            continue;
        }

        // Parse input
        let pos = match input[0].checked_sub(b'1') {
            Some(row) => row as usize,
            None => {
                eprintln!("error: invalid turn");
                continue;
            }
        };

        match Turn::new(player, pos) {
            Some(turn) => return turn,
            None => continue,
        }
    }
}
