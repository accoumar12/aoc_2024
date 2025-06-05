advent_of_code::solution!(2);

fn is_safe_report(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut increasing = None;

    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];

        // Check if difference is within valid range (1-3)
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        // Determine direction on first comparison
        if increasing.is_none() {
            increasing = Some(diff > 0);
        } else {
            // Check if direction is consistent
            let is_increasing = diff > 0;
            if increasing.unwrap() != is_increasing {
                return false;
            }
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let safe_count = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            levels
        })
        .filter(|levels| is_safe_report(levels))
        .count();

    Some(safe_count as u64)
}

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    // First check if it's already safe without removing anything
    if is_safe_report(levels) {
        return true;
    }

    // Try removing each level one at a time
    for i in 0..levels.len() {
        // Create a new vector without the element at index i
        let mut dampened_levels = Vec::with_capacity(levels.len() - 1);
        dampened_levels.extend_from_slice(&levels[..i]);
        dampened_levels.extend_from_slice(&levels[i + 1..]);

        // Check if removing this level makes it safe
        if is_safe_report(&dampened_levels) {
            return true;
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let safe_count = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            levels
        })
        .filter(|levels| is_safe_with_dampener(levels))
        .count();

    Some(safe_count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
