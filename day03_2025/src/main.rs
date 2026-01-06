use aoclib::parse_lines;
use std::io::Error;
use std::str::FromStr;

fn main() {
    let powerbanks: Vec<PowerBank> = parse_lines("./input.txt").unwrap();

    part_1(&powerbanks);
    part_2(&powerbanks);
}

/// Part 1: Find the largest 2-digit number that can be formed by selecting
/// two digits in order from each powerbank, then sum all results.
///
/// Uses an O(n) greedy algorithm: for each digit, try forming a 2-digit number
/// with the maximum digit seen so far, then update the maximum.
///
/// Example: For [9,8,7,6,5,4,3,2,1], we get 98 (9 and 8 in order).
fn part_1(powerbanks: &[PowerBank]) {
    let sum: usize = powerbanks
        .iter()
        .map(|bank| find_largest_two_digit_number(&bank.bank))
        .sum();

    println!("Part 1: {}", sum);
}

/// Part 2: Find the largest 12-digit number that can be formed by selecting
/// 12 digits in order from each powerbank, then sum all results.
///
/// Uses a greedy algorithm that selects the maximum digit at each position
/// while ensuring enough digits remain for subsequent positions.
///
/// Example: For [9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], we get 987654321111.
fn part_2(powerbanks: &[PowerBank]) {
    let sum: usize = powerbanks
        .iter()
        .map(|bank| find_largest_k_digit_number(&bank.bank, 12))
        .sum();

    println!("Part 2: {}", sum);
}

/// Finds the largest 2-digit number by selecting two digits in order.
///
/// Algorithm: Track the maximum first digit seen so far. For each digit,
/// form a 2-digit number with the best first digit, keeping the maximum.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
///
/// # Examples
/// ```
/// assert_eq!(find_largest_two_digit_number(&[9, 8, 7]), 98);
/// assert_eq!(find_largest_two_digit_number(&[8, 1, 9]), 89);
/// assert_eq!(find_largest_two_digit_number(&[1, 2, 3, 4]), 34);
/// ```
fn find_largest_two_digit_number(digits: &[u8]) -> usize {
    if digits.len() < 2 {
        return 0;
    }

    let mut max_two_digit = 0;
    let mut max_first_digit = 0;

    for &digit in digits {
        // Try forming a two-digit number with the best first digit we've seen
        let two_digit = max_first_digit * 10 + digit as usize;
        max_two_digit = max_two_digit.max(two_digit);

        // Update the best first digit we've seen
        max_first_digit = max_first_digit.max(digit as usize);
    }

    max_two_digit
}

/// Finds the largest k-digit number by selecting k digits in order.
///
/// Uses a greedy algorithm:
/// 1. For each position i (0 to k-1), find the maximum digit in a valid range
/// 2. The valid range ensures enough digits remain for positions i+1 to k-1
/// 3. Among multiple occurrences of the max, choose the first (leftmost)
///
/// Time Complexity: O(k * n) where n is the length of digits
/// Space Complexity: O(k)
///
/// # Arguments
/// * `digits` - The sequence of digits to select from
/// * `k` - The number of digits to select
///
/// # Returns
/// The largest k-digit number, or 0 if invalid input
///
/// # Examples
/// ```
/// // From [9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], pick 12 digits
/// assert_eq!(find_largest_k_digit_number(&[9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 12), 987654321111);
///
/// // From [8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], pick 12 digits
/// // Picks the three 8s, then 9, then remaining digits
/// assert_eq!(find_largest_k_digit_number(&[8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 12), 888911112111);
/// ```
fn find_largest_k_digit_number(digits: &[u8], k: usize) -> usize {
    if k == 0 || digits.is_empty() || k > digits.len() {
        return 0;
    }

    let mut result = Vec::with_capacity(k);
    let mut start = 0;

    for position in 0..k {
        // Calculate how many digits we still need after this position
        let remaining = k - position - 1;

        // We can search up to this index while leaving enough digits for later
        let search_end = digits.len() - remaining;

        // Find the maximum digit in the valid range
        let max_digit = *digits[start..search_end]
            .iter()
            .max()
            .expect("search range should not be empty");

        // Find the first occurrence of the maximum digit
        // (choosing first allows more options for later positions)
        let max_idx = digits[start..search_end]
            .iter()
            .position(|&d| d == max_digit)
            .expect("max digit should exist in range");

        result.push(max_digit);
        start = start + max_idx + 1;
    }

    // Convert digit vector to number
    result
        .iter()
        .fold(0, |acc, &digit| acc * 10 + digit as usize)
}

/// Represents a powerbank containing a sequence of digit batteries.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct PowerBank {
    bank: Vec<u8>,
}

impl FromStr for PowerBank {
    type Err = Error;

    /// Parses a string of digits into a PowerBank.
    ///
    /// # Examples
    /// ```
    /// let bank: PowerBank = "123456".parse().unwrap();
    /// assert_eq!(bank.bank, vec![1, 2, 3, 4, 5, 6]);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PowerBank {
            bank: s.chars().map(|ch| ch as u8 - b'0').collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Part 1 Tests =====

    #[test]
    fn test_part1_simple() {
        assert_eq!(find_largest_two_digit_number(&[9, 8, 7]), 98);
        assert_eq!(find_largest_two_digit_number(&[8, 1, 9]), 89);
        assert_eq!(find_largest_two_digit_number(&[1, 2, 3, 4]), 34);
    }

    #[test]
    fn test_part1_all_same() {
        assert_eq!(find_largest_two_digit_number(&[5, 5, 5, 5]), 55);
    }

    #[test]
    fn test_part1_descending() {
        assert_eq!(find_largest_two_digit_number(&[9, 8, 7, 6, 5, 4, 3, 2, 1]), 98);
    }

    #[test]
    fn test_part1_ascending() {
        assert_eq!(find_largest_two_digit_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9]), 89);
    }

    #[test]
    fn test_part1_edge_cases() {
        assert_eq!(find_largest_two_digit_number(&[1, 9]), 19);
        assert_eq!(find_largest_two_digit_number(&[9, 1]), 91);
        assert_eq!(find_largest_two_digit_number(&[]), 0);
        assert_eq!(find_largest_two_digit_number(&[5]), 0);
    }

    // ===== Part 2 Tests =====

    #[test]
    fn test_part2_example_1() {
        // Turn on everything except some 1s at the end
        let bank = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(find_largest_k_digit_number(&bank, 12), 987654321111);
    }

    #[test]
    fn test_part2_example_2() {
        // Turn on everything except some 1s, keeping the 9 at the end
        let bank = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
        assert_eq!(find_largest_k_digit_number(&bank, 12), 811111111119);
    }

    #[test]
    fn test_part2_example_3() {
        // Skip 2, 3, 2 near the start to get larger digits later
        let bank = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        assert_eq!(find_largest_k_digit_number(&bank, 12), 434234234278);
    }

    #[test]
    fn test_part2_example_4() {
        // Pick all three 8s, then continue with remaining digits
        let bank = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        assert_eq!(find_largest_k_digit_number(&bank, 12), 888911112111);
    }

    #[test]
    fn test_part2_all_examples_sum() {
        let banks = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];

        let sum: usize = banks
            .iter()
            .map(|bank| find_largest_k_digit_number(bank, 12))
            .sum();

        assert_eq!(sum, 3121910778619);
    }

    #[test]
    fn test_part2_k_equals_length() {
        // When k equals array length, use all digits in order
        let bank = vec![9, 8, 7, 6, 5];
        assert_eq!(find_largest_k_digit_number(&bank, 5), 98765);
    }

    #[test]
    fn test_part2_small_k() {
        let bank = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(find_largest_k_digit_number(&bank, 1), 9);
        assert_eq!(find_largest_k_digit_number(&bank, 2), 98);
        assert_eq!(find_largest_k_digit_number(&bank, 3), 987);
    }

    #[test]
    fn test_part2_repeated_max_values() {
        // Multiple 9s - should pick first occurrence each time
        let bank = vec![9, 1, 9, 1, 9, 1, 1, 1];
        assert_eq!(find_largest_k_digit_number(&bank, 5), 99911);
    }

    #[test]
    fn test_part2_edge_cases() {
        assert_eq!(find_largest_k_digit_number(&[], 5), 0);
        assert_eq!(find_largest_k_digit_number(&[1, 2, 3], 0), 0);
        assert_eq!(find_largest_k_digit_number(&[1, 2, 3], 5), 0); // k > length
    }

    #[test]
    fn test_part2_all_same_digits() {
        let bank = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(find_largest_k_digit_number(&bank, 4), 7777);
    }

    // ===== PowerBank Parsing Tests =====

    #[test]
    fn test_powerbank_from_str() {
        let bank: PowerBank = "123456789".parse().unwrap();
        assert_eq!(bank.bank, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_powerbank_from_str_single_digit() {
        let bank: PowerBank = "5".parse().unwrap();
        assert_eq!(bank.bank, vec![5]);
    }

    #[test]
    fn test_powerbank_from_str_empty() {
        let bank: PowerBank = "".parse().unwrap();
        assert_eq!(bank.bank, vec![]);
    }

    #[test]
    fn test_powerbank_from_str_zeros() {
        let bank: PowerBank = "1020304".parse().unwrap();
        assert_eq!(bank.bank, vec![1, 0, 2, 0, 3, 0, 4]);
    }

    // ===== Integration Tests =====

    #[test]
    fn test_integration_part1() {
        let banks = vec![
            PowerBank { bank: vec![9, 8, 7, 6, 5, 4, 3, 2, 1] },
            PowerBank { bank: vec![8, 1, 1, 1, 1, 1, 1, 1, 9] },
        ];

        let sum: usize = banks
            .iter()
            .map(|bank| find_largest_two_digit_number(&bank.bank))
            .sum();

        assert_eq!(sum, 98 + 89);
    }

    #[test]
    fn test_integration_part2_small() {
        let banks = vec![
            PowerBank { bank: vec![9, 8, 7] },
            PowerBank { bank: vec![5, 4, 3] },
        ];

        let sum: usize = banks
            .iter()
            .map(|bank| find_largest_k_digit_number(&bank.bank, 2))
            .sum();

        assert_eq!(sum, 98 + 54);
    }
}