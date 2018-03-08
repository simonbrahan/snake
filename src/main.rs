use std::collections::VecDeque;
use std::io;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Location {
    pub x: usize,
    pub y: usize
}

impl Location {
    pub fn new(x: usize, y: usize) -> Location {
        return Location { x: x, y: y };
    }
}

#[derive(Debug)]
struct Game {
    trail: VecDeque<Location>,
    trail_len: usize,
    grid_size: usize,
    player_direction: Direction,
    apple: Location
}

impl Game {
    fn current_head(&self) -> Location {
        let ref current_head = self.trail.front().unwrap();
        return Location::new(current_head.x, current_head.y);
    }
}

fn main() {
    let mut game = Game {
        trail: VecDeque::from(
            vec![
                Location::new(10, 10)
            ]
        ),
        trail_len: 5,
        grid_size: 20,
        player_direction: Direction::Right,
        apple: Location::new(5, 5)
    };

    loop {
        draw(&game);
        println!("Enter a direction: wasd");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
                   .ok()
                   .expect("Failed to read input");

        match input.trim() {
            "w" => game.player_direction = Direction::Up,
            "a" => game.player_direction = Direction::Left,
            "s" => game.player_direction = Direction::Down,
            "d" => game.player_direction = Direction::Right,
            _ => println!("Unrecognised key")
        }

        let current_head = game.current_head();

        game.trail.push_front(
            get_next_loc(&current_head, &game.player_direction)
        );

        game.trail.truncate(game.trail_len);
    }
}

fn draw(game: &Game) {
    // clear the terminal
    println!("{}[2J", 27 as char);

    for y in 0..game.grid_size {
        let mut row = String::new();
        for x in 0..game.grid_size {
            if game.trail.contains(&Location::new(x, y)) {
                row.push('*');
            } else if game.apple == Location::new(x, y) {
                row.push('#');
            } else {
                row.push(' ');
            }
        }

        println!("{}", row);
    }

    println!("Current direction: {:?}", game.player_direction);
}

fn get_next_loc(current_loc: &Location, move_dir: &Direction) -> Location {
    return match move_dir {
        &Direction::Up => Location::new(current_loc.x, current_loc.y - 1),
        &Direction::Down => Location::new(current_loc.x, current_loc.y + 1),
        &Direction::Left => Location::new(current_loc.x - 1, current_loc.y),
        &Direction::Right => Location::new(current_loc.x + 1, current_loc.y)
    };
}
