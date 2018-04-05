extern crate rand;

use std::collections::VecDeque;
use rand::Rng;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn opposite(&self) -> Direction {
        return match self {
            &Direction::Up => Direction::Down,
            &Direction::Down => Direction::Up,
            &Direction::Left => Direction::Right,
            &Direction::Right => Direction::Left
        };
    }
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
    pub apple: Location,
    waiting_time: f64
}

impl Game {
    pub fn new() -> Game {
        let grid_size: usize = 20;

        return Game {
            trail: VecDeque::from(vec![Location::random(&grid_size)]),
            trail_len: 5,
            grid_size: grid_size,
            player_direction: Direction::Right,
            apple: Location::random(&grid_size),
            waiting_time: 0.0
        };
    }

    fn current_head(&self) -> Location {
        let ref current_head = self.trail.front().unwrap();
        return Location::new(current_head.x, current_head.y);
    }

    pub fn change_dir(&mut self, new_dir: Direction) {
        if new_dir == self.player_direction.opposite() {
            return;
        }

        self.player_direction = new_dir;
    }
}

pub fn do_game_move(game: &mut Game, time_change: f64) {

    /*
     * do_game_move is called far more than the desired move rate
     * waitin_time is kept to record when the snake should move
     */
    game.waiting_time += time_change;

    if game.waiting_time < 0.1 {
        return;
    }

    game.waiting_time = 0.0;

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
