use std::fs;

pub fn run() {
    let lines = fs::read_to_string("input/day3.txt").unwrap();
    let lines = lines.lines();

    let mut data: Vec<String> = vec![];
    for line in lines {
        data.push(String::from(line));
    }

    println!("Day 3 - Part 1: {}", power_consumption(&data));
    println!("Day 3 - Part 2: {}", life_support(&data));
}

fn most_common_bit(data: &Vec<String>, n: usize) -> i32 {
    let mut zero = 0;
    let mut one = 0;

    for line in data {
        if line.chars().nth(n).unwrap() == '1' {
            one += 1;
        } else {
            zero += 1;
        }
    }

    if zero > one {
        0
    } else if zero == one {
        -1
    } else {
        1
    }
}

fn power_consumption(data: &Vec<String>) -> i32 {
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for i in 0..data[0].len() {
        let most_common = most_common_bit(&data, i);

        if most_common == 1 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    let gamma = i32::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon = i32::from_str_radix(epsilon.as_str(), 2).unwrap();

    gamma * epsilon
}

fn life_support(data: &Vec<String>) -> i32 {
    let oxygen = get_oxygen(&data);
    let co2 = get_co2(&data);

    let oxygen = i32::from_str_radix(oxygen.as_str(), 2).unwrap();
    let co2 = i32::from_str_radix(co2.as_str(), 2).unwrap();

    oxygen * co2 
}

fn get_oxygen(data: &Vec<String>) -> String {
    let mut data = data.clone();

    for i in 0..data[0].len() {
        if data.len() == 1 {
            break;
        }

        let most_common = most_common_bit(&data, i);

        let mut filter_bit = '0';
        if most_common == 1 || most_common == -1 {
            filter_bit = '1';
        } 

        data = filter_data(data, i, filter_bit);
    }

    String::from(&data[0])
}

fn get_co2(data: &Vec<String>) -> String {
    let mut data = data.clone();

    for i in 0..data[0].len() {
        if data.len() == 1 {
            break;
        }

        let most_common = most_common_bit(&data, i);

        let mut filter_bit = '1';
        if most_common == 1 || most_common == -1 {
            filter_bit = '0';
        } 

        data = filter_data(data, i, filter_bit);
    }

    String::from(&data[0])
}

fn filter_data(data: Vec<String>, n: usize, filter_bit: char) -> Vec<String> {
    data.into_iter().filter(|line| line.chars().nth(n) == Some(filter_bit)).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data: Vec<String> = vec!(
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010")
        );
        let expected = 230;
        let actual = super::life_support(&data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data: Vec<String> = vec!(
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010")
        );
        let expected = 198;
        let actual = super::power_consumption(&data);
        assert_eq!(expected, actual);
    }
}

