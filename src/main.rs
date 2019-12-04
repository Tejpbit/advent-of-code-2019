mod days;

use clap::{App, Arg, SubCommand};
use days::day::Day;
use days::day01;
use days::day02;
use days::day03;
use days::day04;
use std::fs;

fn quit_with_message(message: &str) {
    println!("{}", message);
    std::process::exit(1);
}

/*
*  Featurelist:
* - send specific input file to specific day.
* - run Day trait run function needs to receive path of input
* - extract a parse function from a run function
    - enabling parse to just take a file path and parse it to a format and then call run with it
    - enabling us to start calling run from tests with inputs not defined in files
*/

fn main() {
    let matches = App::new("Advent of Code 2019")
        .version("0.1")
        .author("André Samuelsson")
        .about("Solutions to https://adventofcode.com/2019")
        //        .arg(Arg::with_name("day")
        //            .short("d")
        //            .long("day")
        //            .value_name("NUMBER <1-25>")
        //            .help("Which day to run")
        //            .takes_value(true)
        //        ).
        .subcommand(
            SubCommand::with_name("day")
                .about("Runs the solution for a day")
                .arg(Arg::with_name("DAY_INPUT").index(1).required(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("day") {
        let day_to_run = matches
            .value_of("DAY_INPUT")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let data_file = format!("input_data/day{:02}.txt", day_to_run);

        let contents =
            fs::read_to_string(data_file).expect("Something went wrong reading the file");
        let input = contents.trim();

        match day_to_run {
            1..=25 => println!("Running day {}", day_to_run),
            _ => quit_with_message("Day out of range"),
        }
        let mut a: Vec<Box<dyn Day>> = Vec::new();
        a.push(Box::new(day01::Day01::new()));
        a.push(Box::new(day02::Day02::new()));
        a.push(Box::new(day03::Day03::new()));
        a.push(Box::new(day04::Day04::new()));
        match a.get(day_to_run - 1) {
            Some(day) => {
                match day.part1(input) {
                    Ok(res) => println!("Part1: {}", res),
                    Err(e) => println!("Part1 ERROR: {}", e),
                }
                match day.part2(input) {
                    Ok(res) => println!("Part2: {}", res),
                    Err(e) => println!("Part2 ERROR: {}", e),
                }
            }
            None => quit_with_message("Day not implemented"),
        }
    } else {
        println!("no matches");
    }
}
