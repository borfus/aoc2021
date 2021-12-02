use std::fs;

pub fn run() {
    let lines = fs::read_to_string("input/d1p1.txt").unwrap();
    let lines = lines.lines();

    let mut measurements: Vec<i32> = vec![];
    for line in lines {
        measurements.push(line.parse::<i32>().unwrap());
    }

    println!("Day 1 - Part 1: {}", increase_count(&measurements));
    println!("Day 1 - Part 2: {}", increase_count(&compress_measurements(measurements)));
}

fn increase_count(measurements: &Vec<i32>) -> i32 {
    let mut result = 0;
    let mut prev = measurements[0];

    for i in 1..measurements.len() {
        if measurements[i] > prev {
            result += 1;
        }
        prev = measurements[i];
    }

    result
}

fn compress_measurements(measurements: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    for i in 0..measurements.len()-2 {
        result.push(measurements[i] + measurements[i+1] + measurements[i+2]);
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data: Vec<i32> = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263
        ];
        let expected = 7;
        let actual = super::increase_count(&data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data: Vec<i32> = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263
        ];
        let expected = 5;
        let data = super::compress_measurements(data);
        let actual = super::increase_count(&data);
        assert_eq!(expected, actual);
    }
}

