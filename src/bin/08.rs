advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    struct Antenna {
        position: (usize, usize),
    }
    let count_of_rows = input.lines().filter(|l| !l.trim().is_empty()).count();
    let count_of_columns = input.lines().next()?.len();

    let mut frequency_to_antennas: std::collections::HashMap<char, Vec<Antenna>> = std::collections::HashMap::new();
    for (row_idx, line) in input.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        for (col_idx, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            let antenna = Antenna {
                position: (row_idx, col_idx),
            };
            frequency_to_antennas.entry(char).or_default().push(antenna);
        }
    }
    let mut antinodes_positions: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();

    for (_, antennas) in frequency_to_antennas.iter() {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let x_offset = antennas[j].position.0 as isize - antennas[i].position.0 as isize;
                let y_offset = antennas[j].position.1 as isize - antennas[i].position.1 as isize;
                
                // Antinode on the side of antenna j
                let antinode1_x = antennas[j].position.0 as isize + x_offset;
                let antinode1_y = antennas[j].position.1 as isize + y_offset;
                
                if antinode1_x >= 0 && antinode1_x < count_of_rows as isize && 
                   antinode1_y >= 0 && antinode1_y < count_of_columns as isize {
                    antinodes_positions.insert((antinode1_x as usize, antinode1_y as usize));
                }
                
                // Antinode on the side of antenna i
                let antinode2_x = antennas[i].position.0 as isize - x_offset;
                let antinode2_y = antennas[i].position.1 as isize - y_offset;
                
                if antinode2_x >= 0 && antinode2_x < count_of_rows as isize && 
                   antinode2_y >= 0 && antinode2_y < count_of_columns as isize {
                    antinodes_positions.insert((antinode2_x as usize, antinode2_y as usize));
                }
            }
        }
    }
    Some(antinodes_positions.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    struct Antenna {
        position: (usize, usize),
    }
    let count_of_rows = input.lines().filter(|l| !l.trim().is_empty()).count();
    let count_of_columns = input.lines().next()?.len();

    let mut frequency_to_antennas: std::collections::HashMap<char, Vec<Antenna>> = std::collections::HashMap::new();
    for (row_idx, line) in input.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        for (col_idx, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            let antenna = Antenna {
                position: (row_idx, col_idx),
            };
            frequency_to_antennas.entry(char).or_default().push(antenna);
        }
    }
    let mut antinodes_positions: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();

    for (_, antennas) in frequency_to_antennas.iter() {
        if antennas.len() < 2 {
            continue; // Need at least 2 antennas to form a line
        }
        
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let x_offset = antennas[j].position.0 as isize - antennas[i].position.0 as isize;
                let y_offset = antennas[j].position.1 as isize - antennas[i].position.1 as isize;
                
                // Add the antennas themselves as antinodes
                antinodes_positions.insert(antennas[i].position);
                antinodes_positions.insert(antennas[j].position);
                
                // Find all positions in the direction from i to j
                let mut multiplier = 1;
                loop {
                    let antinode_x = antennas[j].position.0 as isize + x_offset * multiplier;
                    let antinode_y = antennas[j].position.1 as isize + y_offset * multiplier;
                    
                    if antinode_x >= 0 && antinode_x < count_of_rows as isize && 
                       antinode_y >= 0 && antinode_y < count_of_columns as isize {
                        antinodes_positions.insert((antinode_x as usize, antinode_y as usize));
                        multiplier += 1;
                    } else {
                        break;
                    }
                }
                
                // Find all positions in the direction from j to i
                multiplier = 1;
                loop {
                    let antinode_x = antennas[i].position.0 as isize - x_offset * multiplier;
                    let antinode_y = antennas[i].position.1 as isize - y_offset * multiplier;
                    
                    if antinode_x >= 0 && antinode_x < count_of_rows as isize && 
                       antinode_y >= 0 && antinode_y < count_of_columns as isize {
                        antinodes_positions.insert((antinode_x as usize, antinode_y as usize));
                        multiplier += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    Some(antinodes_positions.len() as u64)
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
        assert_eq!(result, Some(34));
    }
}
