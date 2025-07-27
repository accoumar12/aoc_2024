advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    struct Equation {
        target: u64,
        numbers: Vec<u64>,
    }
    // store equations in a vector
    let mut equations: Vec<Equation> = Vec::new();
    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        // retrieve the left and the right from the colon
        println!("Processing line: {}", line);
        let (left, right) = line.split_once(':').unwrap();
        let target = left.trim().parse::<u64>().ok().unwrap();
        let numbers: Vec<u64> = right
            .split(' ')
            .filter_map(|s| s.trim().parse::<u64>().ok())
            .collect();
        let equation = Equation { target, numbers };
        equations.push(equation);
    }
    //print the equations
    for equation in &equations {
        println!("Equation: {} = {:?}", equation.target, equation.numbers);
    }

    let mut sum: u64 = 0;
    for equation in &equations {
        // genereate all permutations of 0 and 1 of the lenght of the count of numbers
        let count_of_operators = equation.numbers.len() - 1;

        for i in 0..(1 << count_of_operators) {
            let mut result = equation.numbers[0];
            for j in 0..count_of_operators {
                let operator = (i >> j) & 1;
                if operator == 0 {
                    result += equation.numbers[j + 1]; // addition
                } else {
                    result *= equation.numbers[j + 1]; // multiplication
                }
            }
            // check if result equals target
            if result == equation.target {
                sum += result;
                break;
            }
        }
    }
    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    struct Equation {
        target: u64,
        numbers: Vec<u64>,
    }
    // store equations in a vector
    let mut equations: Vec<Equation> = Vec::new();
    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (left, right) = line.split_once(':').unwrap();
        let target = left.trim().parse::<u64>().ok().unwrap();
        let numbers: Vec<u64> = right
            .split(' ')
            .filter_map(|s| s.trim().parse::<u64>().ok())
            .collect();
        let equation = Equation { target, numbers };
        equations.push(equation);
    }

    let mut sum: u64 = 0;
    for equation in &equations {
        let count_of_operators = equation.numbers.len() - 1;
        
        // Generate all combinations of 3 operators (0=+, 1=*, 2=||)
        // Use base-3 representation instead of binary
        for i in 0..(3_u32.pow(count_of_operators as u32)) {
            let mut result = equation.numbers[0];
            let mut temp_i = i;
            
            for j in 0..count_of_operators {
                let operator = temp_i % 3;
                temp_i /= 3;
                
                match operator {
                    0 => result += equation.numbers[j + 1], // addition
                    1 => result *= equation.numbers[j + 1], // multiplication
                    2 => {
                        // concatenation: convert both to strings, combine, then parse back
                        let concatenated = format!("{}{}", result, equation.numbers[j + 1]);
                        result = concatenated.parse::<u64>().unwrap();
                    }
                    _ => unreachable!(),
                }
            }
            
            // Check if result equals target
            if result == equation.target {
                sum += result;
                break;
            }
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
