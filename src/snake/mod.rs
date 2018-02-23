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
