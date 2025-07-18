advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    struct Equation {
        target: u32,
        numbers: Vec<u32>,
    }
    // store equations in a vector
    let mut equations: Vec<Equation> = Vec::new();
    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        // retrieve the left and the right from the colon
        let (left, right) = line.split_once(':')?;
        let target = left.trim().parse::<u32>().ok()?;
        let numbers: Vec<u32> = right.split(' ')
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect();
        let equation = Equation { target, numbers };
        equations.push(equation);
    }
    //print the equations
    for equation in &equations {
        println!("Equation: {} = {:?}", equation.target, equation.numbers);
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
