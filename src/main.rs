extern crate snake;
extern crate piston_window;
extern crate num;

use snake::{do_game_move, Direction};
use piston_window::*;
use num::cast::cast;

fn main() {
    let mut game = snake::Game::new();
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [640, 640])
        .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => game.change_dir(Direction::Up),
                Key::Down => game.change_dir(Direction::Down),
                Key::Left => game.change_dir(Direction::Left),
                Key::Right => game.change_dir(Direction::Right),
                _ => ()
            }
        }

        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            for body_part in &game.trail {
                rectangle(
                    [0.5, 0.0, 0.0, 1.0], // red
                    [
                        cast(body_part.x * 32 + 1).unwrap(),
                        cast(body_part.y * 32 + 1).unwrap(),
                        31.0,
                        31.0
                    ],
                    context.transform,
                    graphics
                );
            }

            rectangle(
                [0.0, 0.5, 0.0, 1.0], // green
                [
                    cast(&game.apple.x * 32 + 1).unwrap(),
                    cast(&game.apple.y * 32 + 1).unwrap(),
                    31.0,
                    31.0
                ],
                context.transform,
                graphics
            );
        });

        event.update(|arg| {
            do_game_move(&mut game, arg.dt);
        });
    }
}
