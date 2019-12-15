use super::day::Day;
use itertools::Itertools;
use std::f64::consts::PI;

pub struct Day10 {}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 {}
    }
}

impl Day for Day10 {
    // 25 pixels wide and 6 pixels tall
    fn part1(&self, input: &str) -> Result<String, &str> {

        let mut asteroids = vec![];
        let mut y: f64 = 0f64;
        input.lines().for_each(|row| {
            for (x, character) in row.chars().enumerate() {
                if character == '#' {
                    asteroids.push((x as f64, y));
                }
            }
            y += 1f64;
        }


        );

        let (max, coord) = Day10::calculate_optimal_station_and_available_targets(asteroids);
        let int_coords = (coord.0 as i32, coord.1 as i32);
        Ok(String::from(format!("{:?} {}", int_coords, max)))

    }

    fn part2(&self, input: &str) -> Result<String, &str> {

        let mut asteroids = vec![];
        let mut y: f64 = 0f64;
        input.lines().for_each(|row| {
            for (x, character) in row.chars().enumerate() {
                if character == '#' {
                    asteroids.push((x as f64, y));
                }
            }
            y += 1f64;
        }


        );

        let (_max, asteroid1) = Day10::calculate_optimal_station_and_available_targets(asteroids.clone());
        println!("station: {:?}", asteroid1);

        let mut max = 0;
        let mut coord = (-1f64, -1f64);

        let mut angles_and_distances: Vec<(f64, f64, &(f64, f64))> = asteroids.iter().filter(|a|
            a.0 != asteroid1.0 || a.1 != asteroid1.1
        ).map(|a| {

            let x = a.0 - asteroid1.0;
            let y = a.1 - asteroid1.1;

            let angle = y.atan2(x);
            let distance = (x.powi(2) + y.powi(2)).sqrt();
            (abs_rad_rotate_90_clockwise(angle), distance, a)
        }).collect();

        // sort everything by angle so the group_by buckets things correctly.
        angles_and_distances.sort_by( |x, y| x.0.partial_cmp(&y.0).unwrap() );

        let angle_buckets = angles_and_distances.iter().group_by(|(angle, _distance, _pos)| angle);

        let angle_buckets = angle_buckets.into_iter().map(|(_angle, bucket)| {
            let mut bucket: Vec<&(f64, f64, &(f64,f64))> = bucket.into_iter().collect::<Vec<_>>();
            bucket.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap()); // sort buckets by distance internally

            bucket
        }).collect::<Vec<_>>();
        let mut blast_count = 0;

        let mut bucket_iterators = angle_buckets.iter().map(|b| b.iter()).collect::<Vec<_>>();

        let buckets_count = angle_buckets.clone().into_iter().count();
        let mut last_shot_position = (-1f64, -1f64);
        while blast_count < 200 {
            let current_bucket = blast_count % buckets_count;

            let  a = &mut bucket_iterators[current_bucket];
            last_shot_position = *a.next().unwrap().2;
            blast_count+=1;
        }


        let int_coords = (coord.0 as i32, coord.1 as i32);
        Ok(String::from(format!("{:?}", last_shot_position.0*100f64+last_shot_position.1)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9part1() {
        let day = Day10::new();
        let res = day.part1(".#..#
.....
#####
....#
...##").unwrap();

        assert_eq!(res, String::from("(3, 4) 8"));

        let res = day.part1("......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####").unwrap();
        assert_eq!(res, String::from("(5, 8) 33"));

        let res = day.part1("#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.").unwrap();
        assert_eq!(res, String::from("(1, 2) 35"));


        let res = day.part1(".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..").unwrap();




        assert_eq!(res, String::from("(6, 3) 41"));

        let res = day.part1(".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##").unwrap();




        assert_eq!(res, String::from("(11, 13) 210"));

    }

    #[test]
    fn day10part2() {
        let day = Day10::new();
        let res = day.part2(".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##");

        assert_eq!(String::from("802"), res.unwrap());
    }
}

fn abs_rad_rotate_90_clockwise(rad: f64) -> f64 {
    let mut rad = rad;
    if rad < 0f64 {
        rad += 2f64*PI // only look at positive rads since we want to sort by angle in a counter clockwise manner starting from top
    }
    rad += PI/2f64; // rotate -90 degrees so 0 is above
    if rad >= 2f64*PI {
        rad -= 2f64*PI;
    }
    rad
}

impl Day10 {
    fn calculate_optimal_station_and_available_targets(asteroids: Vec<(f64, f64)>) -> (usize, (f64, f64)) {
        let mut max = 0;
        let mut coord = (-1f64, -1f64);
        for asteroid1 in &asteroids {
            let mut angles: Vec<f64> = asteroids.iter().filter(|a|
                a.0 != asteroid1.0 || a.1 != asteroid1.1
            ).map(|a| {
                let x = a.0 - asteroid1.0;
                let y = a.1 - asteroid1.1;

                let angle = y.atan2(x);
                angle
            }).collect();
            angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
            angles.dedup();


            if angles.len() > max {
                max = angles.len();
                coord = *asteroid1;
            }
        }
        (max, coord)
    }
}
