use aoclib::parse_with;
use std::str::FromStr;

fn main() {
    let ranges: Vec<Range> = parse_with("./input.txt", |content| {
        content
            .split(',')
            .map(|s| Range::from_str(s).map_err(|e| e.into()))
            .collect()
    })
        .unwrap();

    part1(&ranges);
    part2(&ranges);
}

/// Part 1: Find numbers where splitting in half yields two equal parts.
/// Example: 1221 splits into 12 and 21 (not equal), but 1111 splits into 11 and 11 (equal).
fn part1(ranges: &[Range]) {
    let sum: usize = ranges
        .iter()
        .flat_map(|range| range.start..=range.end)
        .filter(|&num| has_mirror_halves(num))
        .sum();

    println!("Part 1: {}", sum);
}

/// Part 2: Find numbers with any repeating pattern of equal-sized chunks.
/// Example: 123123 has pattern "123" repeated twice, 11 has pattern "1" repeated twice.
fn part2(ranges: &[Range]) {
    let sum: usize = ranges
        .iter()
        .flat_map(|range| range.start..=range.end)
        .filter(|&num| has_repeating_pattern(num))
        .sum();

    println!("Part 2: {}", sum);
}

/// Checks if a number has mirror halves (only works for even-length numbers).
/// Example: 1221 -> 12 | 21 (false), 1111 -> 11 | 11 (true)
fn has_mirror_halves(num: usize) -> bool {
    let num_digits = num.ilog10() + 1;

    // Only check numbers with even number of digits
    if num_digits % 2 != 0 {
        return false;
    }

    let half_digits = num_digits / 2;
    let divisor = 10usize.pow(half_digits);

    let first_half = num / divisor;
    let second_half = num % divisor;

    first_half == second_half
}

/// Checks if a number consists of repeating chunks of equal size.
/// Example: 123123 has chunks [123, 123], 777 has chunks [7, 7, 7]
fn has_repeating_pattern(num: usize) -> bool {
    let num_digits = num.ilog10() + 1;

    // Try all possible chunk sizes from 1 to half the number of digits
    for chunk_size in 1..=num_digits / 2 {
        // Skip chunk sizes that don't divide evenly
        if num_digits % chunk_size != 0 {
            continue;
        }

        let divisor = 10usize.pow(chunk_size);
        let first_chunk = num % divisor;

        // Check if all chunks match the first chunk
        let mut temp = num / divisor;
        let mut all_match = true;

        while temp > 0 {
            if temp % divisor != first_chunk {
                all_match = false;
                break;
            }
            temp /= divisor;
        }

        if all_match {
            return true;
        }
    }

    false
}

/// Represents a range of numbers to check (inclusive).
#[derive(Debug, PartialEq)]
struct Range {
    start: usize,
    end: usize,
}

impl FromStr for Range {
    type Err = String;

    /// Parses a range from format "start-end".
    /// Example: "100-200" -> Range { start: 100, end: 200 }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split('-').collect();

        if parts.len() != 2 {
            return Err(format!("Invalid range format: '{}'. Expected 'start-end'", s));
        }

        let start = parts[0]
            .parse()
            .map_err(|_| format!("Invalid start value: '{}'", parts[0]))?;

        let end = parts[1]
            .parse()
            .map_err(|_| format!("Invalid end value: '{}'", parts[1]))?;

        Ok(Range { start, end })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_mirror_halves_basic() {
        // Even length with matching halves
        assert!(has_mirror_halves(1111));
        assert!(has_mirror_halves(2222));
        assert!(has_mirror_halves(1001));

        // Even length without matching halves
        assert!(!has_mirror_halves(1221));
        assert!(!has_mirror_halves(1234));

        // Odd length (always false)
        assert!(!has_mirror_halves(123));
        assert!(!has_mirror_halves(1));
        assert!(!has_mirror_halves(12345));
    }

    #[test]
    fn test_has_mirror_halves_four_digits() {
        assert!(has_mirror_halves(1212));
        assert!(has_mirror_halves(9999));
        assert!(has_mirror_halves(0000));
        assert!(!has_mirror_halves(1234));
        assert!(!has_mirror_halves(5678));
    }

    #[test]
    fn test_has_mirror_halves_six_digits() {
        assert!(has_mirror_halves(123123));
        assert!(has_mirror_halves(999999));
        assert!(!has_mirror_halves(123456));
    }

    #[test]
    fn test_has_repeating_pattern_single_digit() {
        // All single repeating digits
        assert!(has_repeating_pattern(11));
        assert!(has_repeating_pattern(222));
        assert!(has_repeating_pattern(7777));

        // Different digits
        assert!(!has_repeating_pattern(12));
        assert!(!has_repeating_pattern(123));
    }

    #[test]
    fn test_has_repeating_pattern_two_digits() {
        assert!(has_repeating_pattern(1212));
        assert!(has_repeating_pattern(123123));
        assert!(has_repeating_pattern(12341234));

        assert!(!has_repeating_pattern(1234));
        assert!(!has_repeating_pattern(123456));
    }

    #[test]
    fn test_has_repeating_pattern_three_digits() {
        assert!(has_repeating_pattern(123123));
        assert!(has_repeating_pattern(999999));

        assert!(!has_repeating_pattern(123456));
    }

    #[test]
    fn test_has_repeating_pattern_edge_cases() {
        // Single digit numbers don't have a pattern
        assert!(!has_repeating_pattern(1));
        assert!(!has_repeating_pattern(9));

        // Numbers that look like they might have patterns but don't
        assert!(!has_repeating_pattern(1213));
        assert!(!has_repeating_pattern(12312));
    }

    #[test]
    fn test_range_from_str_valid() {
        assert_eq!(
            Range::from_str("100-200").unwrap(),
            Range { start: 100, end: 200 }
        );

        assert_eq!(
            Range::from_str("1-10").unwrap(),
            Range { start: 1, end: 10 }
        );

        // With whitespace
        assert_eq!(
            Range::from_str("  100-200  ").unwrap(),
            Range { start: 100, end: 200 }
        );

        assert_eq!(
            Range::from_str("100-200\n").unwrap(),
            Range { start: 100, end: 200 }
        );
    }

    #[test]
    fn test_range_from_str_invalid() {
        // No dash
        assert!(Range::from_str("100200").is_err());

        // Multiple dashes
        assert!(Range::from_str("100-200-300").is_err());

        // Invalid numbers
        assert!(Range::from_str("abc-200").is_err());
        assert!(Range::from_str("100-xyz").is_err());

        // Empty
        assert!(Range::from_str("").is_err());
    }

    #[test]
    fn test_part1_integration() {
        let ranges = vec![
            Range { start: 10, end: 20 },
            Range { start: 1111, end: 1111 },
        ];

        let sum: usize = ranges
            .iter()
            .flat_map(|range| range.start..=range.end)
            .filter(|&num| has_mirror_halves(num))
            .sum();

        // Only 1111 should match
        assert_eq!(sum, 1111);
    }

    #[test]
    fn test_part2_integration() {
        let ranges = vec![
            Range { start: 11, end: 13 },
            Range { start: 1212, end: 1212 },
        ];

        let sum: usize = ranges
            .iter()
            .flat_map(|range| range.start..=range.end)
            .filter(|&num| has_repeating_pattern(num))
            .sum();

        // 11 and 1212 should match (not 12 or 13)
        assert_eq!(sum, 11 + 1212);
    }

    #[test]
    fn test_comprehensive_small_numbers() {
        // Test all two-digit numbers
        let matching: Vec<usize> = (10..=99)
            .filter(|&n| has_mirror_halves(n))
            .collect();

        // Should be: 11, 22, 33, 44, 55, 66, 77, 88, 99
        assert_eq!(matching.len(), 9);
        assert!(matching.contains(&11));
        assert!(matching.contains(&99));
        assert!(!matching.contains(&12));
    }
}