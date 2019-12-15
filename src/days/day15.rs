use super::day::Day;

pub struct Day15 {}

impl Day15 {
    pub fn new() -> Day15 {
        Day15 {}
    }
}

impl Day for Day15 {
    // 25 pixels wide and 6 pixels tall
    fn part1(&self, _input: &str) -> Result<String, &str> {


        Ok(String::from(format!("{}", "res")))
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
        let day = Day15::new();




    }

    #[test]
    fn day9part2() {
        let day = Day15::new();

    }
}
