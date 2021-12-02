use std::fs;

pub fn run() {
    let lines = fs::read_to_string("input/day2.txt").unwrap();
    let lines = lines.lines();

    let mut data: Vec<Vec<String>> = vec![];
    for line in lines {
        data.push(line.split(' ').map(|s| s.to_string()).collect());
    }

    println!("Day 2 - Part 1: {}", calculate_distance(&data));
    println!("Day 2 - Part 2: {}", calculate_distance_and_aim(&data));
}

fn calculate_distance(data: &Vec<Vec<String>>) -> i32 {
    let mut depth = 0;
    let mut distance = 0;

    for i in data {
        match i[0].as_str() {
            "forward" => distance += i[1].parse::<i32>().unwrap(),
            "down" => depth += i[1].parse::<i32>().unwrap(),
            "up" => depth -= i[1].parse::<i32>().unwrap(),
            _ => println!("Unknown command!")
        };
    }

    depth * distance
}

fn calculate_distance_and_aim(data: &Vec<Vec<String>>) -> i32 {
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;

    for i in data {
        match i[0].as_str() {
            "forward" => {
                distance += i[1].parse::<i32>().unwrap();
                depth += aim * i[1].parse::<i32>().unwrap();
            },
            "down" => aim += i[1].parse::<i32>().unwrap(),
            "up" => aim -= i[1].parse::<i32>().unwrap(),
            _ => println!("Unknown command!")
        };
    }

    depth * distance
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data: Vec<Vec<String>> = vec!(
            vec!(String::from("forward"), String::from("5")),
            vec!(String::from("down"), String::from("5")),
            vec!(String::from("forward"), String::from("8")),
            vec!(String::from("up"), String::from("3")),
            vec!(String::from("down"), String::from("8")),
            vec!(String::from("forward"), String::from("2"))
        );
        let expected = 150;
        let actual = super::calculate_distance(&data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data: Vec<Vec<String>> = vec!(
            vec!(String::from("forward"), String::from("5")),
            vec!(String::from("down"), String::from("5")),
            vec!(String::from("forward"), String::from("8")),
            vec!(String::from("up"), String::from("3")),
            vec!(String::from("down"), String::from("8")),
            vec!(String::from("forward"), String::from("2"))
        );
        let expected = 900;
        let actual = super::calculate_distance_and_aim(&data);
        assert_eq!(expected, actual);
    }
}

