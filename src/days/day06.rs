use super::day::Day;
use std::io::{self, Write};
use std::collections::HashMap;


pub struct Day06 {}

impl Day06 {
    pub fn new() -> Day06 {
        Day06 {}
    }
}

impl Day for Day06 {
    fn part1(&self, input: &str) -> Result<String, &str> {
        let child_orbits_parent: HashMap<_, _> = input
            .lines()
            .map(|s| {
                let parts: Vec<&str> = s.split(")").collect();
                (parts[1], parts[0])
            })
            .collect();


        let mut count = 0;

        for child in child_orbits_parent.keys() {
            let mut current = child_orbits_parent.get(child);
            while current.is_some() {
                current = child_orbits_parent.get(current.unwrap());
                count += 1;
            }
        }


        Ok(String::from(format!("{:?}", count)))
    }

    fn part2(&self, input: &str) -> Result<String, &str> {
        let child_orbits_parent: HashMap<_, _> = input
            .lines()
            .map(|s| {
                let parts: Vec<&str> = s.split(")").collect();
                (parts[1], parts[0])
            })
            .collect();

        let mut you_orbit = HashMap::new();

        let mut previous = "YOU";
        let mut current = child_orbits_parent.get("YOU");
        while current.is_some() {
            you_orbit.insert(current.unwrap(), previous);
            previous = current.unwrap();
            current = child_orbits_parent.get(current.unwrap());
        }

        let mut santas_orbit = HashMap::new();


        let mut previous = "SAN";
        let mut current = child_orbits_parent.get("SAN");
        while current.is_some() {
            santas_orbit.insert(current.unwrap(), previous);
            previous = current.unwrap();
            current = child_orbits_parent.get(current.unwrap());
        }

        let mut you_root = "COM";
        let mut santas_root = "COM";
        while you_root == santas_root {
            you_root = you_orbit.remove(&you_root).unwrap();
            santas_root = santas_orbit.remove(&santas_root).unwrap();
        }

        let x = you_orbit.len() + santas_orbit.len();
        Ok(String::from(format!("{}", x)))
    }
}

fn get_orbits_for(node: &'static str, child_orbits_parent: HashMap<&'static str, &'static str>) -> HashMap<&'static str, &'static str> {
    let mut node_orbits: HashMap<&'static str, &'static str> = HashMap::new();
    let mut previous = node;
    let mut current = child_orbits_parent.get(node);
    while current.is_some() {
        node_orbits.insert(*current.unwrap(), previous);
        previous = current.unwrap();
        current = child_orbits_parent.get(current.unwrap());
    }
    return node_orbits;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let day6 = Day06::new();

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
            K)L";
        let res = day6.part1(input).unwrap();
        assert_eq!(res, String::from("42"));
    }

    #[test]
    fn test_part2() {
        let day6 = Day06::new();
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
