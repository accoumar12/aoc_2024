advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .filter_map(|line| {
            let mut nums = line.split_whitespace().filter_map(|n| n.parse::<u64>().ok());
            Some((nums.next()?, nums.next()?))
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let total_distance: u64 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .filter_map(|line| {
            let mut nums = line.split_whitespace().filter_map(|n| n.parse::<u64>().ok());
            Some((nums.next()?, nums.next()?))
        })
        .unzip();

    let mut freq_map: HashMap<u64, u64> = HashMap::new();
    for num in right {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    let similarity_score: u64 = left
        .iter()
        .map(|&num| num * freq_map.get(&num).copied().unwrap_or(0))
        .sum();

    Some(similarity_score)
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
