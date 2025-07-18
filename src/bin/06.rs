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

    loop {
        let potential_next = state.step(current_pos);
        
        match (
            potential_next.0 < grid.len() && potential_next.1 < grid[0].len(),
            grid.get(potential_next.0).and_then(|row| row.get(potential_next.1))
        ) {
            (false, _) => break, // Out of bounds
            (true, Some('#')) => state = state.next(), // Hit obstacle
            (true, Some(_)) => {
                current_pos = potential_next;
                visited.insert(current_pos);
            }
            _ => unreachable!(),
        }
    }

    Some(visited.len() as u64)
}

// ...existing code...
pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let guard: char = '^';
    let mut start_pos: (usize, usize) = (0, 0);

    for (row_index, line) in input.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        let mut row = Vec::new();
        for (col_index, char) in line.chars().enumerate() {
            row.push(char);
            if char == guard {
                start_pos = (row_index, col_index);
            }
        }
        grid.push(row);
    }
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

    fn causes_loop(grid: &mut Vec<Vec<char>>, start_pos: (usize, usize), obstruction_pos: (usize, usize)) -> bool {
        // Place the obstruction
        grid[obstruction_pos.0][obstruction_pos.1] = '#';
        
        let mut current_pos = start_pos;
        let mut state = State::Up;
        let mut visited_states: HashSet<((usize, usize), State)> = HashSet::new();
        
        loop {
            let state_key = (current_pos, state);
            if visited_states.contains(&state_key) {
                // Remove the obstruction before returning
                grid[obstruction_pos.0][obstruction_pos.1] = '.';
                return true; // Found a loop
            }
            visited_states.insert(state_key);
            
            let potential_next = state.step(current_pos);
            
            match (
                potential_next.0 < grid.len() && potential_next.1 < grid[0].len(),
                grid.get(potential_next.0).and_then(|row| row.get(potential_next.1))
            ) {
                (false, _) => {
                    // Remove the obstruction before returning
                    grid[obstruction_pos.0][obstruction_pos.1] = '.';
                    return false; // Out of bounds, no loop
                }
                (true, Some('#')) => state = state.next(), // Hit obstacle
                (true, Some(_)) => {
                    current_pos = potential_next;
                }
                _ => unreachable!(),
            }
        }
    }

    let mut count = 0;
    
    // Try placing an obstruction at each empty position
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            // Skip if it's the starting position or already an obstacle
            if (row, col) == start_pos || grid[row][col] == '#' {
                continue;
            }
            
            if causes_loop(&mut grid, start_pos, (row, col)) {
                count += 1;
            }
        }
    }
    
    Some(count)
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
        assert_eq!(result, Some(6));
    }
}
