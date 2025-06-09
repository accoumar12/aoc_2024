advent_of_code::solution!(5);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates = Vec::new();
    let mut in_rules = true;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        if in_rules {
            if line.contains('|') {
                let (a, b) = line.split_once('|').unwrap();
                // Parse and store as tuple in rules vector
                let a_num = a.trim().parse().unwrap();
                let b_num = b.trim().parse().unwrap();
                rules.push((a_num, b_num));
            } else {
                in_rules = false;
                updates.push(
                    line.split(',')
                        .map(|s| s.trim().parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                );
            }
        } else {
            updates.push(
                line.split(',')
                    .map(|s| s.trim().parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            );
        }
    }

    // println!("Rules: {:?}", rules);
    // println!("Updates: {:?}", updates);

    let mut sum_of_middle_pages = 0;

    for update in updates {
        // Build a map from page -> index in update, to quickly check order
        let mut positions = HashMap::new();
        for (idx, &page) in update.iter().enumerate() {
            positions.insert(page, idx);
        }

        // Check all rules that apply (both pages in update)
        let mut is_correct = true;
        for &(x, y) in &rules {
            if positions.contains_key(&x) && positions.contains_key(&y) {
                // x must come before y
                if positions[&x] >= positions[&y] {
                    is_correct = false;
                    break;
                }
            }
        }

        if is_correct {
            // Add the middle page
            let middle_page = update[update.len() / 2];
            sum_of_middle_pages += middle_page;
        }
    }

    Some(sum_of_middle_pages)
}



pub fn part_two(input: &str) -> Option<u64> {
    use std::collections::{HashMap, HashSet, VecDeque};

    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates = Vec::new();
    let mut in_rules = true;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        if in_rules {
            if line.contains('|') {
                let (a, b) = line.split_once('|').unwrap();
                rules.push((a.trim().parse().unwrap(), b.trim().parse().unwrap()));
            } else {
                in_rules = false;
                updates.push(
                    line.split(',')
                        .map(|s| s.trim().parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                );
            }
        } else {
            updates.push(
                line.split(',')
                    .map(|s| s.trim().parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            );
        }
    }

    let mut total = 0u64;

    for update in updates {
        let mut positions = HashMap::new();
        for (idx, &page) in update.iter().enumerate() {
            positions.insert(page, idx);
        }

        let mut is_correct = true;
        for &(x, y) in &rules {
            if positions.contains_key(&x) && positions.contains_key(&y) {
                if positions[&x] >= positions[&y] {
                    is_correct = false;
                    break;
                }
            }
        }

        if !is_correct {
            // Build graph for topological sort
            let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
            let mut in_degree: HashMap<u32, usize> = HashMap::new();

            let pages: HashSet<u32> = update.iter().cloned().collect();
            for &p in &pages {
                in_degree.insert(p, 0);
            }

            for &(x, y) in &rules {
                if pages.contains(&x) && pages.contains(&y) {
                    graph.entry(x).or_default().push(y);
                    *in_degree.entry(y).or_default() += 1;
                }
            }

            let mut queue: VecDeque<u32> = in_degree
                .iter()
                .filter(|(_, &deg)| deg == 0)
                .map(|(&k, _)| k)
                .collect();

            let mut sorted = Vec::new();
            while let Some(node) = queue.pop_front() {
                sorted.push(node);
                if let Some(neighbors) = graph.get(&node) {
                    for &neighbor in neighbors {
                        if let Some(deg) = in_degree.get_mut(&neighbor) {
                            *deg -= 1;
                            if *deg == 0 {
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }
            }

            if sorted.len() == pages.len() {
                total += sorted[sorted.len() / 2] as u64;
            } else {
                panic!("Cycle detected or invalid topological sort");
            }
        }
    }

    Some(total)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
