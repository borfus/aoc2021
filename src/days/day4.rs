use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct Board {
    numbers: Vec<Vec<i32>>,
    marked: Vec<Vec<bool>>
}

impl Board {
    fn new() -> Board {
        let mut board = Board {
            numbers: vec![],
            marked: vec![]
        };

        for _ in 0..5 {
            board.marked.push(vec![false; 5]);
        }

        board
    }

    fn mark_number(&mut self, n: &i32) {
        for x in 0..5 {
            for y in 0..5 {
                if self.numbers[x][y] == *n {
                    self.marked[x][y] = true;
                    return;
                }
            }
        }
    }

    fn is_winning_board(&mut self) -> bool {
        // check rows
        for x in 0..5 {
            let mut result = true;
            for y in 0..5 {
                if !self.marked[x][y] {
                    result = false;
                    break;
                }
            }

            if result {
                return true;
            }
        }

        // check columns
        for y in 0..5 {
            let mut result = true;
            for x in 0..5 {
                if !self.marked[x][y] {
                    result = false;
                    break;
                }
            }

            if result {
                return true;
            }
        }

        false
    }

    fn calculate_board_sum(&self, winning_number: &i32) -> i32 {
        let mut unmarked_sum = 0;

        for x in 0..5 {
            for y in 0..5 {
                if !self.marked[x][y] {
                    unmarked_sum += self.numbers[x][y];
                }
            }
        }

        unmarked_sum * winning_number
    }
}

pub fn run() {
    let lines = fs::read_to_string("input/day4.txt").unwrap();
    let lines = lines.lines();

    let mut data: Vec<String> = vec![];
    for line in lines {
        data.push(String::from(line));
    }

    println!("Day 4 - Part 1: {}", find_winning_board_sum(&data));
    println!("Day 4 - Part 2: {}", find_last_winning_board_sum(&data));
}

fn find_winning_board_sum(data: &Vec<String>) -> i32 {
    let numbers: Vec<i32> = data[0].split(',').map(|n| n.parse::<i32>().unwrap()).collect();
    let mut boards: Vec<Board> = vec![];

    let mut i = 2;
    let mut current_board = Board::new();
    while i < data.len() {
        if data[i].is_empty() {
            boards.push(current_board.clone());
            current_board = Board::new();
        } else {
            current_board.numbers.push(data[i].split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect());
        }
        i += 1;
    }

    let mut called_numbers: Vec<i32> = vec![];
    for number in numbers {
        called_numbers.push(number);

        for mut board in boards.clone() {
            for called_number in &called_numbers {
                board.mark_number(&called_number);

                if board.is_winning_board() {
                    return board.calculate_board_sum(&called_number);
                }
            }
        }
    }

    0
}

fn find_last_winning_board_sum(data: &Vec<String>) -> i32 {
    let numbers: Vec<i32> = data[0].split(',').map(|n| n.parse::<i32>().unwrap()).collect();
    let mut boards: Vec<Board> = vec![];

    let mut i = 2;
    let mut current_board = Board::new();
    while i < data.len() {
        if data[i].is_empty() {
            boards.push(current_board.clone());
            current_board = Board::new();
        } else {
            current_board.numbers.push(data[i].split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect());
        }
        i += 1;
    }

    let mut called_numbers: Vec<i32> = vec![];
    for number in numbers {
        called_numbers.push(number);

        let mut end = boards.len();
        for i in 0..end {
            if i >= end {
                break;
            }

            for called_number in &called_numbers {
                boards[i].mark_number(&called_number);

                if boards[i].is_winning_board() {
                    if boards.len() == 1 {
                        return boards[i].calculate_board_sum(&called_number);
                    } else {
                        boards.remove(i);
                        end -= 1;
                        break;
                    }
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data: Vec<String> = vec!(
            String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"),
            String::from(""),
            String::from("22 13 17 11  0"),
            String::from(" 8  2 23  4 24"),
            String::from("21  9 14 16  7"),
            String::from(" 6 10  3 18  5"),
            String::from(" 1 12 20 15 19"),
            String::from(""),
            String::from(" 3 15  0  2 22"),
            String::from(" 9 18 13 17  5"),
            String::from("19  8  7 25 23"),
            String::from("20 11 10 24  4"),
            String::from("14 21 16 12  6"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from(" 2  0 12  3  7"),
            String::from("")
        );
        let expected = 4512;
        let actual = super::find_winning_board_sum(&data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data: Vec<String> = vec!(
            String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"),
            String::from(""),
            String::from("22 13 17 11  0"),
            String::from(" 8  2 23  4 24"),
            String::from("21  9 14 16  7"),
            String::from(" 6 10  3 18  5"),
            String::from(" 1 12 20 15 19"),
            String::from(""),
            String::from(" 3 15  0  2 22"),
            String::from(" 9 18 13 17  5"),
            String::from("19  8  7 25 23"),
            String::from("20 11 10 24  4"),
            String::from("14 21 16 12  6"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from(" 2  0 12  3  7"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from(" 2  0 12  3  7"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from(" 2  0 12  3  7"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from(" 2  0 12  3  7"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from(" 2  0 12  3  7"),
            String::from(""),
        );
        let expected = 1924;
        let actual = super::find_last_winning_board_sum(&data);
        assert_eq!(expected, actual);
    }
}

