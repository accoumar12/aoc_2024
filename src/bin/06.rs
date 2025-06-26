use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
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

    let mut current_pos = start_pos;
    let mut state = State::Up;

    enum State {
        Up,
        Right,
        Down,
        Left,
    }
    impl State {
        fn next(&self) -> Self {
            match self {
                State::Up => State::Right,
                State::Right => State::Down,
                State::Down => State::Left,
                State::Left => State::Up,
            }
        }
        fn step(&self, pos: (usize, usize)) -> (usize, usize) {
            match self {
                State::Up => (pos.0.wrapping_sub(1), pos.1),
                State::Right => (pos.0, pos.1 + 1),
                State::Down => (pos.0 + 1, pos.1),
                State::Left => (pos.0, pos.1.wrapping_sub(1)),
            }
        }
    }
    
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(current_pos);

    while true {
        let potential_next = state.step(current_pos);
        if potential_next.0 >= grid.len() || potential_next.1 >= grid[0].len() {
            break
        }
        else if grid[potential_next.0][potential_next.1] == '#' {
            state = state.next();
            continue;
        }
        else {
            current_pos = potential_next;
            visited.insert(current_pos);
        }
    }

    Some(visited.len() as u64)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
