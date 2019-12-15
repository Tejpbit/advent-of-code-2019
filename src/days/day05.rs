use super::day::Day;
use super::lib::int_computer::read_with_mode_old;
use std::io::{self, Write};

pub struct Day05 {}

impl Day05 {
    pub fn new() -> Day05 {
        Day05 {}
    }
}

impl Day for Day05 {
    fn part1(&self, input: &str) -> Result<String, &str> {
        let input = input
            .split(",")
            .map(|s| s.parse::<i32>().expect("Couldn't read file"))
            .collect::<Vec<i32>>();

        Ok(String::from(format!("{:?}", run_computer(input))))
    }

    fn part2(&self, _input: &str) -> Result<String, &str> {
        Ok(String::from(format!("{}", 2)))
    }
}

fn run_computer(input: Vec<i32>) -> Result<i32, String> {
    let mut memory = input;
    println!("In computer");
    let mut i = 0;
    loop {
        //        println!("mem {:?}", memory);
        let next = memory[i];
        //        println!("i {} next {}", i, next);
        if next == 99 {
            break;
        }
        if next == 1 {
            let arg1_pos = memory[i + 1] as usize;
            let arg2_pos = memory[i + 2] as usize;
            let res_pos = memory[i + 3] as usize;
            let arg1 = memory[arg1_pos];
            let arg2 = memory[arg2_pos];
            memory[res_pos] = arg1 + arg2;
            i += 4;
        } else if next == 2 {
            let arg1_pos = memory[i + 1] as usize;
            let arg2_pos = memory[i + 2] as usize;
            let res_pos = memory[i + 3] as usize;
            let arg1 = memory[arg1_pos];
            let arg2 = memory[arg2_pos];
            memory[res_pos] = arg1 * arg2;
            i += 4;
        } else if next % 100 == 3 {
            let mut input_text = String::new();
            print!("PROGRAM_INPUT  > ");
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            let trimmed = input_text.trim();
            let command_input: i32 = trimmed.parse().unwrap();
            let res_pos = memory[i + 1] as usize;
            memory[res_pos] = command_input;
            i += 2;
        } else if next % 100 == 4 {
            let param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            let arg = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            println!("PROGRAM_OUTPUT > {}", arg);
            i += 2;
        } else if next % 100 == 1 {
            // op 1 (+) with arguments for parameter modes
            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;

            let arg1 = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode_old(memory[i + 2], &memory, param_mode_arg_2 as usize);
            let res_ref = memory[i + 3] as usize;
            memory[res_ref] = arg1 + arg2;
            i += 4;
        } else if next % 100 == 2 {
            // op 2 (*) with arguments for parameter modes
            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;
            let arg1 = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode_old(memory[i + 2], &memory, param_mode_arg_2 as usize);
            let res_ref = memory[i + 3] as usize;
            memory[res_ref] = arg1 * arg2;
            i += 4;
        } else if next % 100 == 5 {
            //        Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.

            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;
            let arg1 = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode_old(memory[i + 2], &memory, param_mode_arg_2 as usize);

            if arg1 != 0 {
                i = arg2 as usize;
            } else {
                i += 3;
            }
        } else if next % 100 == 6 {
            //        Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;
            let arg1 = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode_old(memory[i + 2], &memory, param_mode_arg_2 as usize);

            if arg1 == 0 {
                i = arg2 as usize;
            } else {
                i += 3;
            }
        } else if next % 100 == 7 {
            //        Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;
            let arg1 = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode_old(memory[i + 2], &memory, param_mode_arg_2 as usize);
            let arg3 = memory[i + 3] as usize;

            memory[arg3] = if arg1 < arg2 { 1 } else { 0 };
            i += 4;
        } else if next % 100 == 8 {
            //        Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;
            let arg1 = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode_old(memory[i + 2], &memory, param_mode_arg_2 as usize);
            let arg3 = memory[i + 3] as usize;

            memory[arg3] = if arg1 == arg2 { 1 } else { 0 };
            i += 4;
        } else {
            panic!("Invalid operation")
        }
    }

    return Ok(memory[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_password() {
        let day5 = Day05::new();

        //        let input = "1002,4,3,4,33";
        //        let res = day5.part1(input);

        let input = "3,0,4,0,99";
        let res = day5.part1(input);
        println!("Hello");
        assert!(false);
    }

    #[test]
    fn test_day5_examples() {
        let day5 = Day05::new();
        let a = day5.part1("264360-746325");

        assert_eq!(a, Ok(String::from("123")))
    }
}
