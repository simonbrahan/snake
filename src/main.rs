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
    player_head: Location,
    player_direction: Direction,
    apple: Location
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
        player_head: Location::new(10, 10),
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
