extern crate rand;

use rand::Rng;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl Location {
    fn new(x: usize, y: usize) -> Location {
        Location { x, y }
    }

    fn random(grid_size: usize) -> Location {
        Location::new(
            rand::thread_rng().gen_range(1, grid_size),
            rand::thread_rng().gen_range(1, grid_size),
        )
    }

    fn to_up(&self, grid_size: usize) -> Location {
        let new_y = if self.y == 0 { grid_size - 1 } else { self.y - 1 };

        Location::new(self.x, new_y)
    }

    fn to_down(&self, grid_size: usize) -> Location {
        let new_y = if self.y >= grid_size { 0 } else { self.y + 1};

        Location::new(self.x, new_y)
    }

    fn to_left(&self, grid_size: usize) -> Location {
        let new_x = if self.x == 0 { grid_size - 1 } else { self.x - 1 };

        Location::new(new_x, self.y)
    }

    fn to_right(&self, grid_size: usize) -> Location {
        let new_x = if self.x >= grid_size - 1 { 0 } else { self.x + 1 };

        Location::new(new_x, self.y)
    }
}

#[derive(Debug)]
pub struct Game {
    pub trail: VecDeque<Location>,
    trail_len: usize,
    pub grid_size: usize,
    player_direction: Direction,
    pub apple: Location,
    waiting_time: f64,
}

impl Game {
    pub fn new() -> Game {
        let grid_size: usize = 20;

        Game {
            trail: VecDeque::from(vec![Location::random(grid_size)]),
            trail_len: 5,
            grid_size,
            player_direction: Direction::Right,
            apple: Location::random(grid_size),
            waiting_time: 0.0,
        }
    }

    fn current_head(&self) -> Location {
        let current_head = &self.trail.front().unwrap();
        Location::new(current_head.x, current_head.y)
    }

    pub fn change_dir(&mut self, new_dir: Direction) {
        if new_dir == self.player_direction.opposite() {
            return;
        }

        self.player_direction = new_dir;
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
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
        game.grid_size,
    );

    if next_head == game.apple {
        game.trail_len += 1;
        game.apple = Location::random(game.grid_size);
    }

    if game.trail.contains(&next_head) {
        game.trail_len = 5;
    } else {
        game.trail.push_front(next_head);
    }

    game.trail.truncate(game.trail_len);
}

fn get_next_loc(current_loc: &Location, move_dir: &Direction, grid_size: usize) -> Location {
    match *move_dir {
        Direction::Up => current_loc.to_up(grid_size),
        Direction::Down => current_loc.to_down(grid_size),
        Direction::Left => current_loc.to_left(grid_size),
        Direction::Right => current_loc.to_right(grid_size),
    }
}
