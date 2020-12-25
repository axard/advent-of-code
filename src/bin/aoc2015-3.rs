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

trait MoveTo {
    fn move_to(self, moving: Moving) -> Self;
}

impl MoveTo for Point {
    fn move_to(self, moving: Moving) -> Self {
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

struct Santa;

impl Santa {
    fn parse(input: String) -> usize {
        let mut point = Point { x: 0, y: 0 };

        let mut hm = HashMap::new();
        hm.insert(point, 1);

        for character in input.chars() {
            let result = Moving::parse(character);
            match result {
                Ok(moving) => {
                    point = point.move_to(moving);
                    hm.insert(point, 1);
                }
                Err(error) => println!("parse error: invalid character '{}'", error.character),
            }
        }

        hm.len()
    }
}

struct SantaWithRobot;

impl SantaWithRobot {
    fn parse(input: String) -> usize {
        let mut santa_point = Point { x: 0, y: 0 };
        let mut robot_point = Point { x: 0, y: 0 };

        let mut hm = HashMap::new();
        hm.insert(santa_point, 1);
        hm.insert(robot_point, 1);

        let mut iter = input.chars();
        loop {
            if let Some(santa_command) = iter.next() {
                let result = Moving::parse(santa_command);
                match result {
                    Ok(moving) => {
                        santa_point = santa_point.move_to(moving);
                        hm.insert(santa_point, 1);
                    }
                    Err(error) => println!("parse error: invalid character '{}'", error.character),
                }

                if let Some(robot_command) = iter.next() {
                    let result = Moving::parse(robot_command);
                    match result {
                        Ok(moving) => {
                            robot_point = robot_point.move_to(moving);
                            hm.insert(robot_point, 1);
                        }
                        Err(error) => {
                            println!("parse error: invalid character '{}'", error.character)
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        hm.len()
    }
}

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../../res/aoc2015/3/input.txt"));

    let santa_output = Santa::parse(input.clone().into());
    println!("part 1: {} houses with at least one present", santa_output);

    let santa_with_robot_output = SantaWithRobot::parse(input.clone().into());
    println!(
        "part 2: {} houses with at least one present",
        santa_with_robot_output
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_left() {
        assert_eq!(Santa::parse("<".to_string()), 2);
    }

    #[test]
    fn test_move_right() {
        assert_eq!(Santa::parse(">".to_string()), 2)
    }

    #[test]
    fn test_move_up() {
        assert_eq!(Santa::parse("^".to_string()), 2)
    }

    #[test]
    fn test_move_down() {
        assert_eq!(Santa::parse("v".to_string()), 2)
    }

    #[test]
    fn test_move_square() {
        assert_eq!(Santa::parse("^>v<".to_string()), 4)
    }

    #[test]
    fn test_move_up_and_down() {
        assert_eq!(Santa::parse("^v".to_string()), 2)
    }

    #[test]
    fn test_move_left_and_right() {
        assert_eq!(Santa::parse("<>".to_string()), 2)
    }

    #[test]
    fn test_move_up_and_down_several_times() {
        assert_eq!(Santa::parse("^v^v^v^v^v".to_string()), 2)
    }

    #[test]
    fn test_move_left_with_robot() {
        assert_eq!(SantaWithRobot::parse("<".to_string()), 2);
    }

    #[test]
    fn test_move_right_with_robot() {
        assert_eq!(SantaWithRobot::parse(">".to_string()), 2)
    }

    #[test]
    fn test_move_up_with_robot() {
        assert_eq!(SantaWithRobot::parse("^".to_string()), 2)
    }

    #[test]
    fn test_move_down_with_robot() {
        assert_eq!(SantaWithRobot::parse("v".to_string()), 2)
    }

    #[test]
    fn test_move_square_with_robot() {
        assert_eq!(SantaWithRobot::parse("^>v<".to_string()), 3)
    }

    #[test]
    fn test_move_up_and_down_with_robot() {
        assert_eq!(SantaWithRobot::parse("^v".to_string()), 3)
    }

    #[test]
    fn test_move_left_and_right_with_robot() {
        assert_eq!(SantaWithRobot::parse("<>".to_string()), 3)
    }

    #[test]
    fn test_move_up_and_down_several_times_with_robot() {
        assert_eq!(SantaWithRobot::parse("^v^v^v^v^v".to_string()), 11)
    }
}
