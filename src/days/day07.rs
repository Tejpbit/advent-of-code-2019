use super::lib::int_computer;
use log::{trace};

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
            .map(|s| s.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        let phase_setting: &[i128; 5] = &[0, 1, 2, 3, 4];
        let permutations = permute::permute(phase_setting.to_vec());

        let mut max_output = 0;
        let mut max_phase_setting: Vec<i128> = vec![];
        for permutation in permutations {
            let me_a = channel();
            let a_b = channel();
            let b_c = channel();
            let c_d = channel();
            let d_e = channel();
            let e_me = channel();




            int_computer::run_computer_channels("a",program.clone(), me_a.1, a_b.0.clone());
            int_computer::run_computer_channels("b",program.clone(), a_b.1, b_c.0.clone());
            int_computer::run_computer_channels("c",program.clone(), b_c.1, c_d.0.clone());
            int_computer::run_computer_channels("d",program.clone(), c_d.1, d_e.0.clone());
            int_computer::run_computer_channels("e",program.clone(), d_e.1, e_me.0.clone());

            me_a.0.send(permutation[0]).unwrap();
            me_a.0.send(0).unwrap();
            a_b.0.send(permutation[1]).unwrap();
            b_c.0.send(permutation[2]).unwrap();
            c_d.0.send(permutation[3]).unwrap();
            d_e.0.send(permutation[4]).unwrap();

            let e_out = e_me.1.recv().unwrap();
            if e_out > max_output {
                max_output = e_out;
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
            .map(|s| s.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        let phase_setting = &[5, 6, 7, 8, 9];
        let permutations = permute::permute(phase_setting.to_vec());

        let mut max_output = 0;
        let mut max_phase_setting: Vec<i128> = vec![];
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

            a_b.0.send(permutation[1]).unwrap();
            b_c.0.send(permutation[2]).unwrap();
            c_d.0.send(permutation[3]).unwrap();
            d_e.0.send(permutation[4]).unwrap();

            e_a.0.send(permutation[0]).unwrap();
            e_a.0.send(0).unwrap();

            let last_program_output = observer.1;
            let first_program_input = e_a.0.clone();
            let mut last_recv: i128 = 0;
            for out in last_program_output {
                trace!("Got out {:?}", out);
                last_recv = out;
                first_program_input.send(last_recv).unwrap();
            }
            a.join().unwrap();
            b.join().unwrap();
            c.join().unwrap();
            d.join().unwrap();
            e.join().unwrap();

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
    fn day7part1() {
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
    fn day7part2() {
        let day = Day07::new();
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";

        let res = day.part2(input).unwrap();
        assert_eq!(res, String::from("139629729 [9, 8, 7, 6, 5]"));


        let input =
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";

        let res = day.part2(input).unwrap();
        assert_eq!(res, String::from("18216 [9, 7, 8, 5, 6]"));
    }
}
