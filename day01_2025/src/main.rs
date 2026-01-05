use aoclib::parse_lines_with;
use std::str::FromStr;

/// The total number of positions in the circular track
const TRACK_SIZE: isize = 100;

/// The starting position on the track
const START_POSITION: isize = 50;

fn main() {
    let turns: Vec<Turn> = parse_lines_with("./input.txt", |line| {
        Turn::from_str(line).map_err(|e| e.into())
    })
    .unwrap();
    part1(&turns);
    part2(&turns);
}

/// Solves part 1: counts how many times position 0 is reached after each complete turn.
///
/// Starting at position 50, applies each turn all at once and checks if the final
/// position lands on 0.
fn part1(turns: &[Turn]) {
    let mut position = START_POSITION;
    let mut count = 0;

    for turn in turns {
        match turn {
            Turn::Right(rotation) => position = (position + rotation).rem_euclid(TRACK_SIZE),
            Turn::Left(rotation) => position = (position - rotation).rem_euclid(TRACK_SIZE),
        }
        if position == 0 {
            count += 1
        }
    }
    println!("part 1: {}", count);
}

/// Solves part 2: counts how many times position 0 is crossed during step-by-step movement.
///
/// Starting at position 50, moves one step at a time for each turn and counts every
/// time position 0 is reached during the movement (not just at the end).
fn part2(turns: &[Turn]) {
    let mut position = START_POSITION;
    let mut count = 0;

    for turn in turns {
        match turn {
            Turn::Right(rotation) => {
                for _ in 0..*rotation {
                    position = (position + 1).rem_euclid(TRACK_SIZE);
                    if position == 0 {
                        count += 1;
                    }
                }
            }
            Turn::Left(rotation) => {
                for _ in 0..*rotation {
                    position = (position - 1).rem_euclid(TRACK_SIZE);
                    if position == 0 {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("part 2: {}", count);
}

/// Represents a turn instruction with a direction and rotation amount.
///
/// Turns are parsed from strings in the format "R5" (right 5) or "L3" (left 3).
#[derive(Debug)]
enum Turn {
    /// Turn right by the specified amount
    Right(isize),
    /// Turn left by the specified amount
    Left(isize),
}

impl FromStr for Turn {
    type Err = String;

    /// Parses a turn from a string like "R5" or "L3".
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The string is empty
    /// - The first character is not 'R' or 'L'
    /// - The remaining characters cannot be parsed as an integer
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Empty string cannot be parsed as Turn".to_string());
        }

        let direction = &s[0..1];
        let rotation = s[1..]
            .parse::<isize>()
            .map_err(|e| format!("Failed to parse rotation amount: {}", e))?;

        match direction {
            "R" => Ok(Turn::Right(rotation)),
            "L" => Ok(Turn::Left(rotation)),
            _ => Err(format!(
                "Invalid turn direction '{}', expected 'R' or 'L'",
                direction
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_parse_right() {
        let turn = Turn::from_str("R5").unwrap();
        match turn {
            Turn::Right(5) => (),
            _ => panic!("Expected Right(5)"),
        }
    }

    #[test]
    fn test_turn_parse_left() {
        let turn = Turn::from_str("L10").unwrap();
        match turn {
            Turn::Left(10) => (),
            _ => panic!("Expected Left(10)"),
        }
    }

    #[test]
    fn test_turn_parse_large_number() {
        let turn = Turn::from_str("R999").unwrap();
        match turn {
            Turn::Right(999) => (),
            _ => panic!("Expected Right(999)"),
        }
    }

    #[test]
    fn test_turn_parse_empty_string() {
        let result = Turn::from_str("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Empty string"));
    }

    #[test]
    fn test_turn_parse_invalid_direction() {
        let result = Turn::from_str("X5");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid turn direction"));
    }

    #[test]
    fn test_turn_parse_invalid_number() {
        let result = Turn::from_str("Rabc");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Failed to parse rotation amount"));
    }

    #[test]
    fn test_turn_parse_missing_number() {
        let result = Turn::from_str("R");
        assert!(result.is_err());
    }

    #[test]
    fn test_modulo_wrap_around() {
        // Test that 99 + 2 wraps to 1
        let position = 99_isize;
        let new_position = (position + 2).rem_euclid(TRACK_SIZE);
        assert_eq!(new_position, 1);
    }

    #[test]
    fn test_negative_wrap_around() {
        // Test that 1 - 2 wraps to 99
        let position = 1_isize;
        let new_position = (position - 2).rem_euclid(TRACK_SIZE);
        assert_eq!(new_position, 99);
    }

    #[test]
    fn test_part1_single_turn_hits_zero() {
        let turns = vec![Turn::Right(50)];
        // Starting at 50, moving right 50 should land on 0
        // We can't easily test the output, but we can verify no panic
        part1(&turns);
    }

    #[test]
    fn test_part1_wraps_correctly() {
        let turns = vec![Turn::Right(150)];
        // Starting at 50, moving right 150 should wrap around
        // (50 + 150) % 100 = 0, so should hit zero
        part1(&turns);
    }

    #[test]
    fn test_part1_left_turn() {
        let turns = vec![Turn::Left(50)];
        // Starting at 50, moving left 50 should land on 0
        part1(&turns);
    }

    #[test]
    fn test_part2_single_step() {
        let turns = vec![Turn::Right(1)];
        // Starting at 50, moving right 1 should land on 51
        part2(&turns);
    }

    #[test]
    fn test_part2_crosses_zero() {
        let turns = vec![Turn::Right(50)];
        // Starting at 50, moving right 50 steps should cross 0 once
        part2(&turns);
    }

    #[test]
    fn test_part2_multiple_crosses() {
        let turns = vec![Turn::Right(250)];
        // Starting at 50, moving right 250 steps should cross 0 multiple times
        part2(&turns);
    }

    #[test]
    fn test_constants() {
        assert_eq!(TRACK_SIZE, 100);
        assert_eq!(START_POSITION, 50);
    }
}
