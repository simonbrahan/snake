use std::collections::VecDeque;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Snake {
    pub body: VecDeque<Location>,
    pub length: usize,
    pub current_dir: Direction
}

impl Snake {
    ///
    /// # Examples
    /// ```
    /// use std::collections::VecDeque;
    /// use snake::{Snake, Location, Direction};
    /// let mut snake = Snake {
    ///     body: VecDeque::from(vec![Location::new(0, 0), Location::new(0, 1)]),
    ///     length: 2,
    ///     current_dir: Direction::Left
    /// };
    ///
    /// let expected = Snake {
    ///     body: VecDeque::from(vec![Location::new(-1, 0), Location::new(0, 0)]),
    ///     length: 2,
    ///     current_dir: Direction::Left
    /// };
    ///
    /// snake.move_snake();
    ///
    /// assert_eq!(snake, expected);
    /// ```
    ///
    /// ```
    /// use std::collections::VecDeque;
    /// use snake::{Snake, Location, Direction};
    /// let mut snake = Snake {
    ///     body: VecDeque::from(vec![]),
    ///     length: 2,
    ///     current_dir: Direction::Left
    /// };
    ///
    /// let expected = Snake {
    ///     body: VecDeque::from(vec![Location::new(0, 0)]),
    ///     length: 2,
    ///     current_dir: Direction::Left
    /// };
    ///
    /// snake.move_snake();
    ///
    /// assert_eq!(snake, expected);
    /// ```
    ///
    pub fn move_snake(&mut self) {
        self.body.truncate(self.length - 1);
        self.new_head();
    }

    fn new_head(&mut self) {
        if self.body.is_empty() {
            self.body.push_front(Location::new(0, 0));
        } else {
            let new_front = get_next_loc(self.body.front().unwrap(), &self.current_dir);
            self.body.push_front(new_front);
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Location {
    pub x: i32,
    pub y: i32
}

impl Location {
    ///
    /// Get a Location
    ///
    /// # Examples
    /// ```
    /// use snake::Location;
    /// assert_eq!(Location::new(3, 4), Location { x: 3, y: 4 });
    /// ```
    ///
    pub fn new(x: i32, y: i32) -> Location {
        return Location { x: x, y: y };
    }
}

///
/// Get next location in a direction
///
/// # Examples
/// ```
/// use snake::*;
/// let current_loc = &snake::Location::new(0, 0);
///
/// let current_loc = snake::get_next_loc(&current_loc, &snake::Direction::Up);
/// assert_eq!(snake::Location::new(0, 1), current_loc);
///
/// let current_loc = snake::get_next_loc(&current_loc, &snake::Direction::Right);
/// assert_eq!(snake::Location::new(1, 1), current_loc);
///
/// let current_loc = snake::get_next_loc(&current_loc, &snake::Direction::Down);
/// assert_eq!(snake::Location::new(1, 0), current_loc);
///
/// let current_loc = snake::get_next_loc(&current_loc, &snake::Direction::Left);
/// assert_eq!(snake::Location::new(0, 0), current_loc);
/// ```
///
pub fn get_next_loc(current_loc: &Location, move_dir: &Direction) -> Location {
    return match move_dir {
        &Direction::Up => Location::new(current_loc.x, current_loc.y + 1),
        &Direction::Down => Location::new(current_loc.x, current_loc.y - 1),
        &Direction::Left => Location::new(current_loc.x - 1, current_loc.y),
        &Direction::Right => Location::new(current_loc.x + 1, current_loc.y)
    };
}
