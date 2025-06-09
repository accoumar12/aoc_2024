advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();
    
    let mut count = 0;
    let target = "XMAS";
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];
    
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            for &(dr, dc) in &directions {
                if check_word(&grid, row as i32, col as i32, dr, dc, target) {
                    count += 1;
                }
            }
        }
    }
    
    Some(count)
}

fn check_word(grid: &[Vec<char>], start_row: i32, start_col: i32, dr: i32, dc: i32, word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    
    for (i, &ch) in chars.iter().enumerate() {
        let row = start_row + (i as i32) * dr;
        let col = start_col + (i as i32) * dc;
        
        if row < 0 || row >= grid.len() as i32 || col < 0 || col >= grid[0].len() as i32 {
            return false;
        }
        
        if grid[row as usize][col as usize] != ch {
            return false;
        }
    }
    
    true
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for r in 1..rows-1 {
        for c in 1..cols-1 {
            if grid[r][c] != 'A' {
                continue;
            }

            // Diagonal 1: top-left to bottom-right
            let d1 = [
                grid[r - 1][c - 1],
                grid[r][c],
                grid[r + 1][c + 1],
            ];

            // Diagonal 2: top-right to bottom-left
            let d2 = [
                grid[r - 1][c + 1],
                grid[r][c],
                grid[r + 1][c - 1],
            ];

            let valid_diag = |d: [char; 3]| {
                (d[0] == 'M' && d[1] == 'A' && d[2] == 'S') ||
                (d[0] == 'S' && d[1] == 'A' && d[2] == 'M')
            };

            if valid_diag(d1) && valid_diag(d2) {
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
        assert_eq!(result, Some(18)); 
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
