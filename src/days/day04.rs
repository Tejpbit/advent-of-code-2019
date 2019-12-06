use super::day::Day;

pub struct Day04 {}

impl Day04 {
    pub fn new() -> Day04 {
        Day04 {}
    }
}

impl Day for Day04 {
    fn part1(&self, _input: &str) -> Result<String, &str> {
        let range = 264360..=746325;

        let mut count = 0;
        for i in range {
            if _valid_password(&i.to_string()[..]) {
                count +=1;
            }
        }

        Ok(String::from(format!("{}", count)))
    }

    fn part2(&self, _input: &str) -> Result<String, &str> {
        let range = 264360..=746325;

        let mut count = 0;
        for i in range {
            if _valid_password_2(&i.to_string()[..]) {
                count +=1;
            }
        }

        Ok(String::from(format!("{}", count)))
    }
}

fn _valid_password(password: &str) -> bool {
    let a: Vec<u32> = password.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let two_values_same = a.get(0).unwrap() == a.get(1).unwrap() || a.get(1).unwrap() == a.get(2).unwrap() || a.get(2).unwrap() == a.get(3).unwrap() || a.get(3).unwrap() == a.get(4).unwrap() || a.get(4).unwrap() == a.get(5).unwrap();
    let values_increase = a.get(0).unwrap() <= a.get(1).unwrap() && a.get(1).unwrap() <= a.get(2).unwrap() && a.get(2).unwrap() <= a.get(3).unwrap() && a.get(3).unwrap() <= a.get(4).unwrap() && a.get(4).unwrap() <= a.get(5).unwrap();
    return two_values_same && values_increase;
}

fn _valid_password_2(password: &str) -> bool {
    let a: Vec<u32> = password.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let two_values_same = a.get(0).unwrap() == a.get(1).unwrap() || a.get(1).unwrap() == a.get(2).unwrap() || a.get(2).unwrap() == a.get(3).unwrap() || a.get(3).unwrap() == a.get(4).unwrap() || a.get(4).unwrap() == a.get(5).unwrap();
    let values_increase = a.get(0).unwrap() <= a.get(1).unwrap() && a.get(1).unwrap() <= a.get(2).unwrap() && a.get(2).unwrap() <= a.get(3).unwrap() && a.get(3).unwrap() <= a.get(4).unwrap() && a.get(4).unwrap() <= a.get(5).unwrap();
    let double_not_repeated = (a.get(0).unwrap() == a.get(1).unwrap() && a.get(1).unwrap() != a.get(2).unwrap())
         || (a.get(0).unwrap() != a.get(1).unwrap() && a.get(1).unwrap() == a.get(2).unwrap() && a.get(2).unwrap() != a.get(3).unwrap())
        || (a.get(1).unwrap() != a.get(2).unwrap() && a.get(2).unwrap() == a.get(3).unwrap() && a.get(3).unwrap() != a.get(4).unwrap())
        || (a.get(2).unwrap() != a.get(3).unwrap() && a.get(3).unwrap() == a.get(4).unwrap() && a.get(4).unwrap() != a.get(5).unwrap())
        || (a.get(3).unwrap() != a.get(4).unwrap() && a.get(4).unwrap() == a.get(5).unwrap());
    return two_values_same && values_increase && double_not_repeated;
}

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_invalid_password() {
        let day3 = Day04::new();

        assert!(!_valid_password("442573"));
        assert!(_valid_password("111111"));
        assert!(!_valid_password("223450"));
        assert!(!_valid_password("123789"));

        assert!(_valid_password_2("112233"));
        assert!(!_valid_password_2("123444"));
        assert!(_valid_password_2("111122"));
    }

    #[test]
    fn test_day4_examples() {
        let day3 = Day04::new();
        let a = day3.part1(
            "264360-746325",
        );

        assert_eq!(a, Ok(String::from("123")))
    }


}
