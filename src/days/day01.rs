use std::fs;
use super::day::Day;

pub struct Day01 {}

impl Day01{
    pub fn new() -> Day01 {
        Day01{}
    }
}

impl Day for Day01 {
    fn run(&self) {
        let contents = fs::read_to_string("input_data/day01.txt")
            .expect("Something went wrong reading the file");
        let input = contents.trim();

        let input = input.lines().map(|s| {
            s.parse::<i32>().expect("asd")
        } ).collect::<Vec<i32>>();

        let a: i32 = input.iter().map(|i| calculate_fuel_req_part1(*i)).sum();
        println!("Fuelreq a {}",a);

        let b: i32 = input.iter().map(|i| calculate_fuel_req_part2(*i)).sum();

        println!("Fuelreq b: {}", b);
    }
}



fn calculate_fuel_req_part1(i: i32) -> i32 {
    i/3-2
}

fn calculate_fuel_req_part2(i: i32) -> i32 {
    let mut fuel_req = calculate_fuel_req_part1(i);
    let mut remainder_mass= fuel_req;

    while remainder_mass > 0 {
        remainder_mass = calculate_fuel_req_part1(remainder_mass);
        if remainder_mass > 0 {
            fuel_req += remainder_mass;
        }
    }
    fuel_req
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {

        assert_eq!(calculate_fuel_req_part1(100756), 33583);
        assert_eq!(calculate_fuel_req_part2(100756), 50346);
    }
}
