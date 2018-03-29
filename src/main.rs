extern crate snake;

use std::io;
use snake::{Game, accept_input, do_game_move};

fn main() {
    let mut game = snake::Game::new();

    loop {
        draw(&game);
        println!("Enter a direction: wasd");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
                   .ok()
                   .expect("Failed to read input");

        accept_input(&mut game, input.trim());
        do_game_move(&mut game);
    }
}

fn draw(game: &Game) {
    // clear the terminal
    println!("{}[2J", 27 as char);

    for y in 0..game.grid_size {
        let mut row = String::new();
        for x in 0..game.grid_size {
            if game.snake_on(x, y) {
                row.push('*');
            } else if game.apple_on(x, y) {
                row.push('#');
            } else {
                row.push(' ');
            }
        }

        println!("{}", row);
    }
}
