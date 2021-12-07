use std::fs;
use std::i32;

pub fn run() {
    let data = fs::read_to_string("input/day7.txt").unwrap();

    println!("Day 7 - Part 1: {}", least_fuel(&data, false));
    println!("Day 7 - Part 2: {}", least_fuel(&data, true));
}

fn least_fuel(data: &String, increasing: bool) -> i32 {
    let crabs: Vec<i32> = data.trim().split(',').map(|n| n.parse::<i32>().unwrap()).collect();

    let (mut min, mut max) = (i32::MAX, i32::MIN);
    for crab in &crabs {
        if *crab > max {
            max = *crab;
        }

        if *crab < min {
            min = *crab;
        }
    }

    let mut total_cost = i32::MAX;

    for n in min..=max {
        let mut new_cost = 0;
        for i in 0..crabs.len() {
            if !increasing {
                new_cost += (crabs[i] - n).abs();
            } else {
                new_cost += (crabs[i] - n).abs() * ((crabs[i] - n).abs() + 1) / 2;
            }
        }

        if new_cost < total_cost {
            total_cost = new_cost;
        }
    }

    total_cost
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data = String::from("16,1,2,0,4,2,7,1,2,14");
        let expected = 37;
        let actual = super::least_fuel(&data, false);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data = String::from("16,1,2,0,4,2,7,1,2,14");
        let expected = 168;
        let actual = super::least_fuel(&data, true);
        assert_eq!(expected, actual);
    }
}

