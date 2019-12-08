use super::lib::int_computer;
use log::{info, trace, warn};

use super::day::Day;
use log;
use permute;
use std::sync::mpsc::channel;

pub struct Day07 {}

impl Day07 {
    pub fn new() -> Day07 {
        Day07 {}
    }
}

impl Day for Day07 {
    fn part1(&self, input: &str) -> Result<String, &str> {
        let program = input
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let phase_setting = &[0, 1, 2, 3, 4];
        let permutations = permute::permute(phase_setting.to_vec());

        let mut max_output = 0;
        let mut max_phase_setting: Vec<i32> = vec![];
        for permutation in permutations {
            let a = int_computer::run_computer(program.clone(), vec![permutation[0], 0]);
            let b = int_computer::run_computer(program.clone(), vec![permutation[1], a[0]]);
            let c = int_computer::run_computer(program.clone(), vec![permutation[2], b[0]]);
            let d = int_computer::run_computer(program.clone(), vec![permutation[3], c[0]]);
            let e = int_computer::run_computer(program.clone(), vec![permutation[4], d[0]]);
            if e[0] > max_output {
                max_output = e[0];
                max_phase_setting = permutation;
            }
        }
        Ok(String::from(format!(
            "{:?} {:?}",
            max_output, max_phase_setting
        )))
    }

    fn part2(&self, input: &str) -> Result<String, &str> {
        let program = input
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let phase_setting = &[5, 6, 7, 8, 9];
        let permutations = permute::permute(phase_setting.to_vec());

        let mut max_output = 0;
        let mut max_phase_setting: Vec<i32> = vec![];
        for permutation in permutations {
            let a_b = channel();
            let b_c = channel();
            let c_d = channel();
            let d_e = channel();
            let e_a = channel();
            let observer = channel();

            let a = int_computer::run_computer_channels("A", program.clone(), e_a.1, a_b.0.clone());
            let b = int_computer::run_computer_channels("B", program.clone(), a_b.1, b_c.0.clone());
            let c = int_computer::run_computer_channels("C", program.clone(), b_c.1, c_d.0.clone());
            let d = int_computer::run_computer_channels("D", program.clone(), c_d.1, d_e.0.clone());
            let e = int_computer::run_computer_channels(
                "E",
                program.clone(),
                d_e.1,
                observer.0.clone(),
            );

            a_b.0.send(Some(permutation[1]));
            b_c.0.send(Some(permutation[2]));
            c_d.0.send(Some(permutation[3]));
            d_e.0.send(Some(permutation[4]));

            e_a.0.send(Some(permutation[0]));
            e_a.0.send(Some(0));

            let last_program_output = observer.1;
            let first_program_input = e_a.0.clone();
            let mut last_recv: i32 = 0;
            for out in last_program_output {
                trace!("Got out {:?}", out);
                match out {
                    Some(o) => {
                        last_recv = o;
                        first_program_input.send(Some(last_recv));
                    }
                    None => break,
                };
            }
            a.join();
            b.join();
            c.join();
            d.join();
            e.join();

            if max_output < last_recv {
                max_output = last_recv;
                max_phase_setting = permutation;
            }
        }
        Ok(String::from(format!(
            "{:?} {:?}",
            max_output, max_phase_setting
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let day = Day07::new();

        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let res = day.part1(input).unwrap();
        assert_eq!(res, String::from("43210 [4, 3, 2, 1, 0]"));

        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let res = day.part1(input).unwrap();
        assert_eq!(res, String::from("54321 [0, 1, 2, 3, 4]"));

        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let res = day.part1(input).unwrap();
        assert_eq!(res, String::from("65210 [1, 0, 4, 3, 2]"));
    }

    #[test]
    fn test_part2_day7() {
        let day = Day07::new();
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";

        let res = day.part2(input).unwrap();
        assert_eq!(res, String::from("139629729 [9, 8, 7, 6, 5]"));
    }
}
