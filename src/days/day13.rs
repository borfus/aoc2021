use std::fs;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

pub fn run() {
    let lines = fs::read_to_string("input/day13.txt").unwrap();
    let lines = lines.lines().collect();

    println!("Day 13 - Part 1: {}", count_dots(&lines, true));
    let _ = count_dots(&lines, false);
}

fn count_dots(data: &Vec<&str>, just_first: bool) -> i32 {
    let mut result = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    let mut points: Vec<Point> = vec![];
    let mut fold_instructions: Vec<&str> = vec![];

    let mut max_x = 0;
    let mut max_y = 0;

    let mut i = 0;
    while data[i] != "" {
        let lines: Vec<&str> = data[i].split(',').collect();
        let point = Point {
            x: lines[0].parse().unwrap(),
            y: lines[1].parse().unwrap(),
        };

        if point.x > max_x {
            max_x = point.x;
        }

        if point.y > max_y {
            max_y = point.y;
        }

        points.push(point);

        i += 1;
    }

    i += 1;
    while i < data.len() {
        fold_instructions.push(data[i]);
        i += 1;
    }

    for _ in 0..=max_y {
        grid.push(vec!['.'; (max_x + 1) as usize]);
    }

    for point in points {
        grid[point.y as usize][point.x as usize] = '#';
    }

    for fold in fold_instructions {
        let fold: Vec<&str> = fold.split('=').collect();
        if fold[0] == "fold along y" {
            grid = fold_y(&grid, fold[1].parse().unwrap());
        } else if fold[0] == "fold along x" {
            grid = fold_x(&grid, fold[1].parse().unwrap());
        }

        if just_first {
            break;
        }
    }

    for row in &grid {
        for x in row {
            if *x == '#' {
                result += 1
            }
        }
    }

    if !just_first {
        println!("Day 13 - Part 2:");
        for row in grid {
            println!("{:?}", row);
        }
    }

    result
}

fn fold_y(grid: &Vec<Vec<char>>, y_fold: i32) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = vec![];

    for y in 0..y_fold as usize {
        let mut row: Vec<char> = vec![];
        for x in 0..grid[0].len() {
            row.push(grid[y][x]);
        }
        new_grid.push(row);
    }

    for y in (y_fold+1) as usize..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '#' {
                new_grid[(y as i32 - y_fold * 2).abs() as usize][x] = '#';
            }
        }
    }

    new_grid
}

fn fold_x(grid: &Vec<Vec<char>>, x_fold: i32) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = vec![];

    for y in 0..grid.len() {
        let mut row: Vec<char> = vec![];
        for x in 0..x_fold as usize {
            row.push(grid[y][x]);
        }
        new_grid.push(row);
    }

    for y in 0..grid.len() {
        for x in (x_fold+1) as usize..grid[0].len() {
            if grid[y][x] == '#' {
                new_grid[y][(x as i32 - x_fold * 2).abs() as usize] = '#';
            }
        }
    }

    new_grid
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data: Vec<&str> = vec![
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5"
        ];
        let expected = 17;
        let actual = super::count_dots(&data, true);
        assert_eq!(expected, actual);

        let expected = 16;
        let actual = super::count_dots(&data, false);
        assert_eq!(expected, actual);
    }
}

