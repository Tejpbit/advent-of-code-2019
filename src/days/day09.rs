use super::day::Day;
use super::lib::int_computer::run_computer_channels;
use std::sync::mpsc::channel;

pub struct Day09 {}

impl Day09 {
    pub fn new() -> Day09 {
        Day09 {}
    }
}

impl Day for Day09 {
    // 25 pixels wide and 6 pixels tall
    fn part1(&self, input: &str) -> Result<String, &str> {

        let input = input.split(",").map(|x| x.parse::<i128>().unwrap()).collect();

        let (program_inputter, program_input) = channel();
        let (program_output, program_output_receiver) = channel();
        run_computer_channels("part1 day9", input, program_input, program_output);

        program_inputter.send(1).unwrap();


        let mut output: Vec<i128> = vec![];
        for o in program_output_receiver {
            println!("got {:?}", o);
            output.push(o)
        }

        //let output: Vec<i128> = program_output_receiver.iter().inspect(|x| println!("inspect: {:?}", x)).filter(|x| x.is_some()).map(|x| x.unwrap()).collect();
        let res: Vec<String> = output.iter().map(|x| x.to_string()).collect();

        let res = res.join(",");
        println!("res is |{}|", res);

//        let res = program_output_receiver.recv().unwrap();
        Ok(String::from(format!("{}", res)))
    }

    fn part2(&self, _input: &str) -> Result<String, &str> {


        Ok(String::from(format!("{:?}", "Answer above")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9part1() {
        let day = Day09::new();

        let input = "104,1125899906842624,99";
        let res = day.part1(input).unwrap();
        assert_eq!(res, String::from("1125899906842624"));


        let input = "1102,34915192,34915192,7,4,7,99,0";
        let res = day.part1(input).unwrap();
        assert_eq!(res, String::from("1219070632396864"));


        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let res = day.part1(input).unwrap();
        assert_eq!(res, String::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"));


    }

    #[test]
    fn day9part2() {
        let day6 = Day09::new();
        let input = "\
                     COM)B\n\
                     B)C\n\
                     C)D\n\
                     D)E\n\
                     E)F\n\
                     B)G\n\
                     G)H\n\
                     D)I\n\
                     E)J\n\
                     J)K\n\
                     K)L\n\
                     K)YOU\n\
                     I)SAN";

        let res = day6.part2(input).unwrap();
        assert_eq!(res, String::from("4"));
    }
}
