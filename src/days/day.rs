pub trait Day {
    fn part1(&self, input: &str) -> Result<String, &str>;
    fn part2(&self, input: &str) -> Result<String, &str>;
}
