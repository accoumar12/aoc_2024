advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    use regex::Regex;
    
    // Pattern to match mul(X,Y) where X and Y are 1-3 digit numbers
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").ok()?;
    
    let mut sum = 0u64;
    
    for cap in re.captures_iter(input) {
        let x: u64 = cap[1].parse().ok()?;
        let y: u64 = cap[2].parse().ok()?;
        sum += x * y;
    }
    
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    use regex::Regex;
    
    // Pattern to match mul(X,Y), do(), or don't()
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").ok()?;
    
    let mut sum = 0u64;
    let mut enabled = true; // mul instructions start enabled
    
    for cap in re.captures_iter(input) {
        let full_match = &cap[1];
        
        match full_match {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if full_match.starts_with("mul") && enabled => {
                // This is a mul instruction and we're enabled
                let x: u64 = cap[2].parse().ok()?;
                let y: u64 = cap[3].parse().ok()?;
                sum += x * y;
            }
            _ => {} // mul instruction but disabled, or something else
        }
    }
    
    Some(sum)
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
