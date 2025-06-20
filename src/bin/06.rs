advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
// ...existing code...
    let mut grid: Vec<Vec<char>> = Vec::new();
    let guard: char = '^';
    let mut start_pos: (usize, usize) = (0, 0);

    for (row_index, line) in input.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        let mut row = Vec::new();
        for (col_index, char) in line.chars().enumerate() {
            row.push(char);  // Add each character to the current row
            if char == guard {
                start_pos = (row_index, col_index);  // Update start_pos if guard is found
            }
        }
        grid.push(row);  // Add the completed row to the grid
    }
    println!("Grid: {:?}", grid);
    println!("Start position: {:?}", start_pos);

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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
