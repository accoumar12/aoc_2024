advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    struct Antenna {
        position: (usize, usize),
        frequency: char,
    }

    let mut frequency_to_antennas: std::collections::HashMap<char, Vec<Antenna>> = std::collections::HashMap::new();
    for (row_idx, line) in input.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        for (col_idx, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            let antenna = Antenna {
                position: (row_idx, col_idx),
                frequency: char,
            };
            frequency_to_antennas.entry(char).or_default().push(antenna);
        }
    }


    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
