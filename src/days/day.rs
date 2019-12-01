pub trait DayParse<T> {
    fn parse(path: &str) -> T;
}

pub trait Day {
    fn run(&self);
}