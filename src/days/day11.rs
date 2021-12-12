use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

pub fn run() {
    let lines = fs::read_to_string("input/day11.txt").unwrap();
    let lines = lines.lines().collect();

    println!("Day 11 - Part 1: {}", count_flashes(&lines, 100, false));
    println!("Day 11 - Part 2: {}", count_flashes(&lines, 100, true));
}

fn count_flashes(data: &Vec<&str>, steps: i32, check_for_all_flash: bool) -> i32 {
    let mut result = 0;

    let octopus_grid: Rc<RefCell<Vec<Vec<i32>>>> = Rc::new(RefCell::new(vec![]));
    for line in data {
        let mut row: Vec<i32> = vec![];
        for c in line.chars() {
            row.push(c.to_string().parse().unwrap());
        }
        octopus_grid.borrow_mut().push(row);
    }

    let rows = octopus_grid.borrow().len();
    let columns = octopus_grid.borrow()[0].len();

    let mut step = 0;
    let mut finished = false;
    while !finished {
        step += 1;
        for row in 0..rows {
            for column in 0..columns {
                octopus_grid.borrow_mut()[row][column] += 1;
            }
        }

        for row in 0..rows {
            for column in 0..columns {
                if octopus_grid.borrow()[row][column] == 10 {
                    result += flash(Rc::clone(&octopus_grid), 0, row, column);
                }
            }
        }

        if !check_for_all_flash {
            if step == steps {
                return result;
            }
        }

        finished = true;
        for row in 0..10 {
            for column in 0..10 {
                if octopus_grid.borrow()[row][column] == -1 {
                    octopus_grid.borrow_mut()[row][column] = 0;
                } else {
                    finished = false;
                }
            }
        }

        if finished && check_for_all_flash {
            return step;
        }
    }

    result
}

fn flash(octopus_grid: Rc<RefCell<Vec<Vec<i32>>>>, mut flashes: i32, x: usize, y: usize) -> i32 {
    flashes += 1;
    octopus_grid.borrow_mut()[x][y] = -1;

    for next_x in 0..=2 {
        for next_y in 0..=2 {
            let next_x: i32 = x as i32 + next_x as i32 - 1;
            let next_y: i32 = y as i32 + next_y as i32 - 1;

            if (next_x >= 0 && next_x < 10) && (next_y >= 0 && next_y < 10) {
                let neighbor_x = next_x as usize;
                let neighbor_y = next_y as usize;

                if octopus_grid.borrow()[neighbor_x][neighbor_y] != -1 {
                    octopus_grid.borrow_mut()[neighbor_x][neighbor_y] += 1;
                    if octopus_grid.borrow()[neighbor_x][neighbor_y] >= 10 {
                        flashes = flash(Rc::clone(&octopus_grid), flashes, neighbor_x, neighbor_y);
                    }
                }
            }
        }
    }

    flashes
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data: Vec<&str> = vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526"
        ];
        let expected = 1656;
        let actual = super::count_flashes(&data, 100, false);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data: Vec<&str> = vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526"
        ];
        let expected = 195;
        let actual = super::count_flashes(&data, 100, true);
        assert_eq!(expected, actual);
    }
}

