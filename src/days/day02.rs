use std::fs;
use super::day::Day;


pub struct Day02 {}

impl Day02 {
    pub fn new() -> Day02 {
        Day02 {}
    }
}

impl Day for Day02 {
    fn run(&self) {
        let contents = fs::read_to_string("input_data/day02.txt")
            .expect("Something went wrong reading the file");
        let input = contents.trim();

        let input = input.split(",").map(|s| {
            s.parse::<usize>().expect("Couldn't read file")
        } ).collect::<Vec<usize>>();

        println!("result part1 {}", run_computer(input.clone(), 12, 1));

        for i in 1..=99 {
            for j in 1..=99 {

                if 19690720 == run_computer(input.clone(), i, j) {
                    println!("noun {} verb {}", i, j);
                    return;
                }

            }
        }
        println!("No match found");
    }
}


fn run_computer(input: Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut input = input;
    input[1] = noun;
    input[2] = verb;

    let mut i = 0;
    loop  {
        let next = input[i];

        if next == 99 {
            break;
        }
        if next == 1 {
            let arg1_pos = input[i+1];
            let arg2_pos = input[i+2];
            let res_pos = input[i+3];
            let arg1 = input[arg1_pos];
            let arg2 = input[arg2_pos];
            input[res_pos] = arg1+arg2;
        } else if next == 2 {
            let arg1_pos = input[i+1];
            let arg2_pos = input[i+2];
            let res_pos = input[i+3];
            let arg1 = input[arg1_pos];
            let arg2 = input[arg2_pos];
            input[res_pos] = arg1*arg2;
        }
        i += 4;
    }

    return input[0];
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() {
        let a = Day02::new();
        a.run();


        assert_eq!(true, false);
    }
}
