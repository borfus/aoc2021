use std::fs;
use std::collections::HashMap;

pub fn run() {
    let data = fs::read_to_string("input/day6.txt").unwrap();

    println!("Day 5 - Part 1: {}", count_fish(&data, 80));
    println!("Day 5 - Part 2: {}", count_fish(&data, 256));
}

fn count_fish(data: &String, days: i32) -> i64 {
    let fish: Vec<i32> = data.split_whitespace().collect::<String>().split(',').collect::<Vec<&str>>().iter().map(|n| n.parse::<i32>().unwrap()).collect();

    let mut cycles = HashMap::new();

    for f in fish {
        let value = cycles.entry(f).or_insert(0);
        *value += 1;
    }

    for _ in 0..days {
        let mut new_cycles = HashMap::new();
        for (k, v) in cycles.iter_mut() {
            if *k == 0 {
                let mut value = new_cycles.entry(6).or_insert(0);
                *value += *v;
                value = new_cycles.entry(8).or_insert(0);
                *value = *v;
            } else {
                let value = new_cycles.entry(*k-1).or_insert(0);
                *value += *v;
            }
        }
        cycles = new_cycles;
    }

    let mut result = 0;
    for (_, v)  in cycles {
        result += v;
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data = String::from("3,4,3,1,2");
        let expected = 26;
        let actual = super::count_fish(&data, 18);
        assert_eq!(expected, actual);
        let expected = 5934;
        let actual = super::count_fish(&data, 80);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data = String::from("3,4,3,1,2");
        let expected = 26984457539;
        let actual = super::count_fish(&data, 256);
        assert_eq!(expected, actual);
    }
}

