use super::day::Day;
use std::str::from_utf8;

pub struct Day08 {}

impl Day08 {
    pub fn new() -> Day08 {
        Day08 {}
    }
}

impl Day for Day08 {
    // 25 pixels wide and 6 pixels tall
    fn part1(&self, input: &str) -> Result<String, &str> {
        let input = input.trim();

        let width = 25;
        let height = 6;

        let number_of_rows = input.len() / width;
        let number_of_layers = number_of_rows / height;
        let layer_length = input.len() / number_of_layers;

        let layers = input
            .as_bytes()
            .chunks(layer_length)
            .map(from_utf8)
            .map(|l| {
                let a = l
                    .unwrap()
                    .chars()
                    //.inspect(|a| println!("empty? {}", a))
                    .map(|d| d.to_digit(10).unwrap())
                    .collect::<Vec<u32>>();
                a
            })
            .collect::<Vec<Vec<u32>>>();

        let layer_with_fewest_zeros =
            layers
                .iter()
                .fold((100, vec![]), |(fewest_zeroes, layer), current_layer| {
                    let zeroes_in_layer = current_layer.iter().filter(|x| **x == 0).count();
                    if zeroes_in_layer < fewest_zeroes {
                        (zeroes_in_layer, current_layer.to_vec())
                    } else {
                        (fewest_zeroes, layer)
                    }
                });
        let one_count = layer_with_fewest_zeros
            .1
            .iter()
            .filter(|x| **x == 1)
            .count();
        let two_count = layer_with_fewest_zeros
            .1
            .iter()
            .filter(|x| **x == 2)
            .count();

        let res = one_count * two_count;

        Ok(String::from(format!("{:?}", res)))
    }

    fn part2(&self, input: &str) -> Result<String, &str> {
        let input = input.trim();

        let width = 25;
        let height = 6;

        let number_of_rows = input.len() / width;
        let number_of_layers = number_of_rows / height;
        let layer_length = input.len() / number_of_layers;

        let layers = input
            .as_bytes()
            .chunks(layer_length)
            .map(from_utf8)
            .map(|l| {
                let a = l
                    .unwrap()
                    .chars()
                    //.inspect(|a| println!("empty? {}", a))
                    .map(|d| d.to_digit(10).unwrap())
                    .collect::<Vec<u32>>();
                a
            })
            .collect::<Vec<Vec<u32>>>();

        let image = layers.iter().fold(vec![2; 150], |acc, cur| {
            acc.iter()
                .zip(cur)
                .map(|(above, below)| if *above == 2 { *below } else { *above })
                .collect()
        });

        image.chunks(width).for_each(|row| {
            for cell in row {
                print!(
                    "{}",
                    match cell {
                        0 => "■",
                        1 => "☐",
                        _ => panic!("invalid color"),
                    }
                )
            }
            println!();
        });

        Ok(String::from(format!("{:?}", "Answer above")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let day = Day08::new();

        let input = "123456789012";
        let res = day.part1(input).unwrap();
        assert_eq!(res, String::from("1"));
    }

    #[test]
    fn test_part2() {
        let day6 = Day08::new();
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
