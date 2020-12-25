use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

enum Moving {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct ParseError {
    character: char,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError")
    }
}

impl Moving {
    fn parse(c: char) -> Result<Moving, ParseError> {
        match c {
            '^' => Ok(Moving::Up),
            'v' => Ok(Moving::Down),
            '<' => Ok(Moving::Left),
            '>' => Ok(Moving::Right),
            _ => Err(ParseError { character: c }),
        }
    }
}

trait Move {
    fn r#move(self, moving: Moving) -> Self;
}

impl Move for Point {
    fn r#move(self, moving: Moving) -> Self {
        match moving {
            Moving::Up => Point {
                x: self.x,
                y: self.y + 1,
            },
            Moving::Down => Point {
                x: self.x,
                y: self.y - 1,
            },
            Moving::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Moving::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

fn parse(input: String) -> usize {
    let mut point = Point { x: 0, y: 0 };

    let mut hm = HashMap::new();
    hm.insert(point, 1);

    for character in input.chars() {
        let result = Moving::parse(character);
        match result {
            Ok(moving) => {
                point = point.r#move(moving);
                hm.insert(point, 1);
            }
            Err(error) => println!("parse error: invalid character '{}'", error.character),
        }
    }

    hm.len()
}

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../../res/aoc2015/3/input.txt"));
    let output = parse(input.into());

    println!("part 1: {} houses with at least one present", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_left() {
        assert_eq!(parse("<".to_string()), 2);
    }

    #[test]
    fn test_move_right() {
        assert_eq!(parse(">".to_string()), 2)
    }

    #[test]
    fn test_move_up() {
        assert_eq!(parse("^".to_string()), 2)
    }

    #[test]
    fn test_move_down() {
        assert_eq!(parse("v".to_string()), 2)
    }

    #[test]
    fn test_move_square() {
        assert_eq!(parse("^>v<".to_string()), 4)
    }

    #[test]
    fn test_move_up_and_down() {
        assert_eq!(parse("^v".to_string()), 2)
    }

    #[test]
    fn test_move_left_and_right() {
        assert_eq!(parse("<>".to_string()), 2)
    }

    #[test]
    fn test_move_up_and_down_several_times() {
        assert_eq!(parse("^v^v^v^v^v".to_string()), 2)
    }
}
