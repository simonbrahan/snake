extern crate rand;

use std::collections::VecDeque;
use rand::Rng;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Location {
    pub x: usize,
    pub y: usize
}

impl Location {
    fn new(x: usize, y: usize) -> Location {
        return Location { x: x, y: y };
    }

    fn random(grid_size: &usize) -> Location {
        return Location::new(
            rand::thread_rng().gen_range(1, *grid_size),
            rand::thread_rng().gen_range(1, *grid_size)
        );
    }

    fn to_up(&self, grid_size: &usize) -> Location {
        let new_y;

        if self.y == 0 {
            new_y = grid_size - 1;
        } else {
            new_y = self.y - 1;
        }

        return Location::new(self.x, new_y);
    }

    fn to_down(&self, grid_size: &usize) -> Location {
        let new_y;

        if self.y >= *grid_size - 1 {
            new_y = 0;
        } else {
            new_y = self.y + 1;
        }

        return Location::new(self.x, new_y);
    }

    fn to_left(&self, grid_size: &usize) -> Location {
        let new_x;

        if self.x == 0 {
            new_x = *grid_size - 1;
        } else {
            new_x = self.x - 1;
        }

        return Location::new(new_x, self.y);
    }

    fn to_right(&self, grid_size: &usize) -> Location {
        let new_x;

        if self.x >= *grid_size - 1 {
            new_x = 0;
        } else {
            new_x = self.x + 1;
        }

        return Location::new(new_x, self.y);
    }
}

#[derive(Debug)]
pub struct Game {
    pub trail: VecDeque<Location>,
    trail_len: usize,
    pub grid_size: usize,
    player_direction: Direction,
    apple: Location
}

impl Game {
    pub fn new() -> Game {
        let grid_size: usize = 20;

        return Game {
            trail: VecDeque::from(vec![Location::random(&grid_size)]),
            trail_len: 5,
            grid_size: grid_size,
            player_direction: Direction::Right,
            apple: Location::random(&grid_size)
        };
    }

    fn current_head(&self) -> Location {
        let ref current_head = self.trail.front().unwrap();
        return Location::new(current_head.x, current_head.y);
    }

    pub fn snake_on(&self, x: usize, y: usize) -> bool {
        return self.trail.contains(&Location::new(x, y));
    }

    pub fn apple_on(&self, x: usize, y: usize) -> bool {
        return self.apple == Location::new(x, y);
    }
}

pub fn accept_input(game: &mut Game, input: &str) {
    match input {
        "w" => game.player_direction = Direction::Up,
        "a" => game.player_direction = Direction::Left,
        "s" => game.player_direction = Direction::Down,
        "d" => game.player_direction = Direction::Right,
        _ => ()
    }
}

pub fn do_game_move(game: &mut Game) {
    let next_head = get_next_loc(
        &game.current_head(),
        &game.player_direction,
        &game.grid_size
    );

    if next_head == game.apple {
        game.trail_len += 1;
        game.apple = Location::random(&game.grid_size);
    }

    if game.trail.contains(&next_head) {
        game.trail_len = 5;
    } else {
        game.trail.push_front(next_head);
    }

    game.trail.truncate(game.trail_len);
}

fn get_next_loc(current_loc: &Location, move_dir: &Direction, grid_size: &usize) -> Location {
    return match move_dir {
        &Direction::Up => current_loc.to_up(&grid_size),
        &Direction::Down => current_loc.to_down(&grid_size),
        &Direction::Left => current_loc.to_left(&grid_size),
        &Direction::Right => current_loc.to_right(&grid_size)
    };
}
