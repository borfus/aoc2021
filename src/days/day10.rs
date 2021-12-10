use std::fs;

pub fn run() {
    let lines = fs::read_to_string("input/day10.txt").unwrap();
    let lines = lines.lines();

    let mut data: Vec<String> = vec![];
    for line in lines {
        data.push(String::from(line));
    }

    println!("Day 10 - Part 1: {}", find_line_score(&data, false));
    println!("Day 10 - Part 2: {}", find_line_score(&data, true));
}

fn find_line_score(data: &Vec<String>, ignore_corrupt: bool) -> i64 {
    let mut result = 0;
    let mut incomplete_line_scores: Vec<i64> = vec![];
    let mut lines: Vec<Vec<char>> = vec![];

    for line in data {
        lines.push(line.chars().collect());
    }

    for line in lines {
        let mut input: Vec<char> = vec![];
        let mut next_expected_close: Vec<char> = vec![];
        let mut illegal_char = ' '; 

        let mut corrupt = false;
        for c in line {
            let inverse = match_inverse(c);
            match c {
                '(' | '[' | '{' | '<' => {
                    input.push(c);
                    next_expected_close.push(inverse.unwrap());
                },
                ')' | ']' | '}' | '>' => {
                    let top = next_expected_close.pop().unwrap();
                    if top != c {
                        illegal_char = c;
                        next_expected_close.push(top);
                        corrupt = true;
                        break;
                    }
                },
                _ => println!("Invalid character found!")
            }
        }

        if !ignore_corrupt {
            let line_score = match illegal_char {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0
            };

            result += line_score;
        } else {
            if corrupt {
                continue;
            }

            let mut line_result: i64 = 0;
            for c in next_expected_close.into_iter().rev() {
                line_result *= 5;
                match c {
                    ')' => line_result += 1,
                    ']' => line_result += 2,
                    '}' => line_result += 3,
                    '>' => line_result += 4,
                    _ => line_result += 0
                }
            }
            incomplete_line_scores.push(line_result);
        }
    }

    if ignore_corrupt {
        incomplete_line_scores.sort();
        result = incomplete_line_scores[incomplete_line_scores.len()/2];
    }

    result
}

fn match_inverse(c: char) -> Option<char> {
    match c {
        '(' => Some(')'), 
        '[' => Some(']'), 
        '{' => Some('}'), 
        '<' => Some('>'), 
        ')' => Some('('), 
        ']' => Some('['), 
        '}' => Some('{'), 
        '>' => Some('<'),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data: Vec<String> = vec![
            String::from("[({(<(())[]>[[{[]{<()<>>"),
            String::from("[(()[<>])]({[<{<<[]>>("),
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"),
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"),
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]")
        ];
        let expected = 26397;
        let actual = super::find_line_score(&data, false);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data: Vec<String> = vec![
            String::from("[({(<(())[]>[[{[]{<()<>>"),
            String::from("[(()[<>])]({[<{<<[]>>("),
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"),
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"),
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]")
        ];
        let expected = 288957;
        let actual = super::find_line_score(&data, true);
        assert_eq!(expected, actual);
    }
}

