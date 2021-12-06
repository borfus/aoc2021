use std::fs;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Pair {
    p1: Point,
    p2: Point
}

impl Pair {
    fn new(p1: Point, p2: Point) -> Pair {
        Pair { p1, p2 }
    }
}

#[derive(Debug)]
struct Matrix {
    grid: Vec<Vec<i32>>
}

impl Matrix {
    fn new(max: i32) -> Matrix {
        let mut ret = Matrix {
            grid: vec![]
        };

        for _ in 0..=max {
            ret.grid.push(vec![0; (max+1) as usize]);
        }

        ret
    }

    fn draw_line(&mut self, pair: Pair, diagonal: bool) {
        let p1 = pair.p1;
        let p2 = pair.p2;

        // if diagonal
        if p1.x != p2.x && p1.y != p2.y {
            if diagonal {
                self.draw_diag_line(Pair::new(p1, p2));
            }
            return;
        }

        let left;
        let right;
        let vertical;
        if p1.x == p2.x {
            // vertical line
            vertical = true;
            if p1.y < p2.y {
                left = p1.y;
                right = p2.y;
            } else {
                left = p2.y;
                right = p1.y;
            }
        } else {
            // horizontal line
            vertical = false;
            if p1.x < p2.x {
                left = p1.x;
                right = p2.x;
            } else {
                left = p2.x;
                right = p1.x;
            }
        }

        for i in left..=right {
            if vertical {
                self.grid[p1.x as usize][i as usize] += 1;
            } else {
                self.grid[i as usize][p1.y as usize] += 1;
            }
        }
    }

    fn draw_diag_line(&mut self, pair: Pair) {
        let p1 = pair.p1;
        let p2 = pair.p2;

        let mut points: Vec<Point> = vec![];
        let mut x: Vec<i32>;
        let mut y: Vec<i32>;

        if p1.x < p2.x {
            x = (p1.x..=p2.x).collect();
        } else {
            x = (p2.x..=p1.x).collect();
            x.reverse();
        }

        if p1.y < p2.y {
            y = (p1.y..=p2.y).collect();
        } else {
            y = (p2.y..=p1.y).collect();
            y.reverse();
        }

        for i in 0..x.len() {
            points.push(Point::new(x[i], y[i]));
        }

        for point in points {
            self.grid[point.x as usize][point.y as usize] += 1;
        }
    }

    fn count_dangerous(&self) -> i32 {
        let mut result = 0;

        for x in 0..self.grid.len() {
            for y in 0..self.grid.len() {
                if self.grid[x][y] >= 2 {
                    result += 1;
                }
            }
        }

        result
    }
}

pub fn run() {
    let lines = fs::read_to_string("input/day5.txt").unwrap();
    let lines = lines.lines();

    let mut data: Vec<String> = vec![];
    for line in lines {
        data.push(String::from(line));
    }

    println!("Day 5 - Part 1: {}", find_danger_spots(&data, false));
    println!("Day 5 - Part 2: {}", find_danger_spots(&data, true));
}

fn find_danger_spots(data: &Vec<String>, diagonal: bool) -> i32 {
    let mut pairs: Vec<Pair> = vec![];

    let mut max_val = 0;
    for line in data {
        let split: Vec<&str> = line.split(' ').collect();
        let p1_split: Vec<&str> = split[0].split(',').collect();
        let p2_split: Vec<&str> = split[2].split(',').collect();
        let p1 = Point::new(p1_split[0].parse::<i32>().unwrap(), p1_split[1].parse::<i32>().unwrap());
        let p2 = Point::new(p2_split[0].parse::<i32>().unwrap(), p2_split[1].parse::<i32>().unwrap());
        let pair = Pair::new(p1, p2);

        if pair.p1.x > max_val {
            max_val = pair.p1.x;
        } else if pair.p1.y > max_val {
            max_val = pair.p1.y;
        } else if pair.p2.x > max_val {
            max_val = pair.p2.x;
        } else if pair.p2.y > max_val {
            max_val = pair.p2.y;
        }

        pairs.push(pair);
    }

    let mut matrix = Matrix::new(max_val);

    for pair in pairs {
        matrix.draw_line(pair, diagonal);
    }

    matrix.count_dangerous()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data: Vec<String> = vec!(
            String::from("0,9 -> 5,9"),
            String::from("8,0 -> 0,8"),
            String::from("9,4 -> 3,4"),
            String::from("2,2 -> 2,1"),
            String::from("7,0 -> 7,4"),
            String::from("6,4 -> 2,0"),
            String::from("0,9 -> 2,9"),
            String::from("3,4 -> 1,4"),
            String::from("0,0 -> 8,8"),
            String::from("5,5 -> 8,2")
        );
        let expected = 5;
        let actual = super::find_danger_spots(&data, false);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data: Vec<String> = vec!(
            String::from("0,9 -> 5,9"),
            String::from("8,0 -> 0,8"),
            String::from("9,4 -> 3,4"),
            String::from("2,2 -> 2,1"),
            String::from("7,0 -> 7,4"),
            String::from("6,4 -> 2,0"),
            String::from("0,9 -> 2,9"),
            String::from("3,4 -> 1,4"),
            String::from("0,0 -> 8,8"),
            String::from("5,5 -> 8,2")
        );
        let expected = 12;
        let actual = super::find_danger_spots(&data, true);
        assert_eq!(expected, actual);
    }
}

