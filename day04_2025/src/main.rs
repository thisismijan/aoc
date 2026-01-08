use std::collections::HashSet;

fn main() {
    let rolls = parse_input(aoclib::read_input("./input.txt").unwrap());

    part_1(&rolls);
    part_2(rolls);
}

/// Parses the input string and returns a set of coordinates where '@' symbols appear.
///
/// # Arguments
/// * `input` - A string containing a grid where '@' marks positions of interest
///
/// # Returns
/// A `HashSet` of (row, column) coordinates as `(isize, isize)` tuples
fn parse_input(input: String) -> HashSet<(isize, isize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '@')
                .map(move |(col, _)| (row as isize, col as isize))
        })
        .collect()
}

/// Solves Part 1: Counts positions with fewer than 4 neighbors (accessible positions).
fn part_1(input: &HashSet<(isize, isize)>) {
    println!("Part 1: {}", find_accessible(input).len());
}

/// Solves Part 2: Repeatedly removes accessible positions until none remain,
/// counting the total number of positions removed.
fn part_2(mut input: HashSet<(isize, isize)>) {
    let mut total_removed = 0;

    loop {
        let acc = find_accessible(&input);
        if acc.is_empty() {
            break;
        }
        total_removed += acc.len();
        // More efficient than calling remove() for each element
        input.retain(|pos| !acc.contains(pos));
    }

    println!("Part 2: {}", total_removed);
}

/// Finds all "accessible" positions - those with fewer than 4 neighbors
/// in the 8 surrounding cells (including diagonals).
///
/// # Arguments
/// * `input` - A set of grid positions to check
///
/// # Returns
/// A vector of positions that have fewer than 4 neighbors
fn find_accessible(input: &HashSet<(isize, isize)>) -> Vec<(isize, isize)> {
    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    input
        .iter()
        .filter(|&&(row, col)| {
            DIRECTIONS
                .iter()
                .filter(|&&(dr, dc)| input.contains(&(row + dr, col + dc)))
                .count()
                < 4
        })
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_basic() {
        let input = "\
.@.
@.@
.@.";
        let rolls = parse_input(input.to_string());
        assert_eq!(rolls.len(), 4);
        assert!(rolls.contains(&(0, 1)));
        assert!(rolls.contains(&(1, 0)));
        assert!(rolls.contains(&(1, 2)));
        assert!(rolls.contains(&(2, 1)));
    }

    #[test]
    fn test_parse_input_empty() {
        let input = "\
...
...
...";
        let rolls = parse_input(input.to_string());
        assert_eq!(rolls.len(), 0);
    }

    #[test]
    fn test_find_accessible_isolated() {
        // Single isolated position
        let mut rolls = HashSet::new();
        rolls.insert((0, 0));

        let accessible = find_accessible(&rolls);
        assert_eq!(accessible.len(), 1, "Isolated position should be accessible");
    }

    #[test]
    fn test_find_accessible_cross_pattern() {
        // Cross pattern: center has 4 neighbors
        let input = "\
.@.
@@@
.@.";
        let rolls = parse_input(input.to_string());
        let accessible = find_accessible(&rolls);

        // Center has exactly 4 neighbors, so NOT accessible
        // Only the 4 edge positions (with 1 neighbor each) are accessible
        assert_eq!(accessible.len(), 4);
        assert!(accessible.contains(&(0, 1)));
        assert!(accessible.contains(&(1, 0)));
        assert!(accessible.contains(&(1, 2)));
        assert!(accessible.contains(&(2, 1)));
    }

    #[test]
    fn test_find_accessible_dense_cluster() {
        // 3x3 grid - center has 8 neighbors
        let input = "\
@@@
@@@
@@@";
        let rolls = parse_input(input.to_string());
        let accessible = find_accessible(&rolls);

        // Only corner and edge positions have < 4 neighbors
        // Corners: 3 neighbors each
        // Edges: 5 neighbors each
        // Center: 8 neighbors
        assert_eq!(accessible.len(), 4, "Only corners have < 4 neighbors");
    }

    #[test]
    fn test_part_1_example() {
        let input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let rolls = parse_input(input.to_string());
        let accessible = find_accessible(&rolls);

        assert_eq!(accessible.len(), 13, "Expected 13 accessible positions");
    }

    #[test]
    fn test_part_1_example_total_count() {
        let input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let rolls = parse_input(input.to_string());
        assert_eq!(rolls.len(), 70, "Should parse 70 @ symbols");
    }

    #[test]
    fn test_part_2_removal_sequence() {
        // Simple case where all positions are eventually removed
        let input = "\
@@@
@@@
@@@";
        let rolls = parse_input(input.to_string());

        let mut input_copy = rolls.clone();
        let mut total_removed = 0;
        let mut iterations = 0;

        loop {
            let acc = find_accessible(&input_copy);
            if acc.is_empty() {
                break;
            }
            total_removed += acc.len();
            input_copy.retain(|pos| !acc.contains(pos));
            iterations += 1;
        }

        assert_eq!(total_removed, 9, "All 9 positions should be removed");
        assert!(iterations > 1, "Should take multiple iterations");
    }

    #[test]
    fn test_part_2_single_position() {
        let mut rolls = HashSet::new();
        rolls.insert((0, 0));

        let mut total_removed = 0;
        loop {
            let acc = find_accessible(&rolls);
            if acc.is_empty() {
                break;
            }
            total_removed += acc.len();
            rolls.retain(|pos| !acc.contains(pos));
        }

        assert_eq!(total_removed, 1);
    }

    #[test]
    fn test_neighbor_count_boundary() {
        // Test the < 4 boundary condition
        // Position with exactly 3 neighbors: should be accessible
        let input = "\
.@.
@@.
...";
        let rolls = parse_input(input.to_string());
        let accessible = find_accessible(&rolls);

        // All positions have < 4 neighbors
        assert_eq!(accessible.len(), 3);
    }
}