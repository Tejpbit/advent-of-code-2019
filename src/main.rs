mod days;

use clap::{App, Arg, SubCommand};
use days::day::Day;
use days::day01;
use days::day02;
use days::day03;
use days::day04;
use days::day05;
use days::day06;
use days::day07;
use days::day08;
use env_logger;
use std::fs;

fn quit_with_message(message: &str) {
    println!("{}", message);
    std::process::exit(1);
}

fn main() {
    env_logger::init();
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
        a.push(Box::new(day05::Day05::new()));
        a.push(Box::new(day06::Day06::new()));
        a.push(Box::new(day07::Day07::new()));
        a.push(Box::new(day08::Day08::new()));
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
