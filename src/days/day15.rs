use std::fs;

pub fn run() {
    let lines = fs::read_to_string("input/day15.txt").unwrap();
    let lines = lines.lines().collect();

    println!("Day 15 - Part 1: {}", find_path(&lines, 1));
}

fn find_path(data: &Vec<&str>, n_tiles: i32) -> i32 {
    let mut result = 0;
    let mut grid: Vec<Vec<i32>> = vec![];

    for line in data {
        let mut row: Vec<i32> = vec![];
        for c in line.chars() {
            row.push(c.to_string().parse().unwrap());
        }
        grid.push(row);
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data: Vec<&str> = vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581"
        ];
        let expected = 40;
        let actual = super::find_path(&data, 1);
        assert_eq!(expected, actual);
    }
}

