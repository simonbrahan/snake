extern crate snake;
extern crate piston_window;
extern crate num;

use std::{io, thread, time};
use snake::{Game, accept_input, do_game_move};
use piston_window::*;
use num::cast::cast;

fn main() {
    let mut game = snake::Game::new();
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [640, 640])
        .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            for body_part in &game.trail {
                rectangle(
                    [0.5, 0.0, 0.0, 1.0], // red
                    [
                        cast(body_part.x * 32).unwrap(),
                        cast(body_part.y * 32).unwrap(),
                        32.0,
                        32.0
                    ],
                    context.transform,
                    graphics
                );
            }
        });

        // Update the state of the game
        event.update(|arg| {
            do_game_move(&mut game, arg.dt);
        });
    }
}
