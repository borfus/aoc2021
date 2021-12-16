use std::{fs, collections::VecDeque};

pub fn run() {
    let lines = fs::read_to_string("input/day15.txt").unwrap();
    let lines = lines.lines().collect();

    println!("Day 15 - Part 1: {}", find_path(&lines, 1));
    println!("Day 15 - Part 2: {}", find_path(&lines, 5));
}

fn find_path(data: &Vec<&str>, tiles: usize) -> i32 {
    let mut grid: Vec<Vec<i32>> = vec![];

    for line in data {
        let mut row: Vec<i32> = vec![];
        for c in line.chars() {
            row.push(c.to_string().parse().unwrap());
        }
        grid.push(row);
    }

    let columns = grid.len();
    let rows = grid[0].len();

    let mut cost: Vec<Vec<Option<i32>>> = vec![];
    for _ in 0..rows*tiles {
        cost.push(vec![None; columns*tiles]);
    }

    let mut visited: VecDeque<(i32, usize, usize)> = VecDeque::new();
    visited.push_back((0, 0, 0));

    while visited.len() > 0 {
        let (distance, row, column) = visited.pop_front().unwrap();

        let mut val = grid[row % rows][column % columns] + (row / rows) as i32 + (column / columns) as i32;
        while val > 9 {
            val -= 9;
        }
        let pos_cost = distance + val;

        if cost[row][column] == None || pos_cost < cost[row][column].unwrap() {
            cost[row][column] = Some(pos_cost);
        } else {
            continue;
        }

        let next_rows: Vec<i32> = vec![-1, 0, 1, 0];
        let next_cols: Vec<i32> = vec![0, 1, 0, -1];
        for n in 0..4 {
            if ((row == 0 && next_rows[n] == -1) || (column == 0 && next_cols[n] == -1)) 
                || ((row == rows * tiles - 1 && next_rows[n] == 1) || (column == columns * tiles - 1 && next_cols[n] == 1)) {
                continue;
            }

            let next_row = row as i32 + next_rows[n];
            let next_col = column as i32 + next_cols[n];
            visited.push_back((cost[row][column].unwrap(), next_row as usize, next_col as usize));
        }
    }

    cost[tiles * rows - 1][tiles * columns - 1].unwrap() - grid[0][0]
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

    #[test]
    fn part2() {
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
        let expected = 315;
        let actual = super::find_path(&data, 5);
        assert_eq!(expected, actual);
    }
}

