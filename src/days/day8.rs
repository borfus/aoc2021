use std::fs;
use std::collections::HashMap;
use std::i32;

pub fn run() {
    let lines = fs::read_to_string("input/day8.txt").unwrap();
    let lines = lines.lines();

    let mut data: Vec<String> = vec![];
    for line in lines {
        data.push(String::from(line));
    }

    println!("Day 8 - Part 1: {}", unique_output_digits(&data));
    println!("Day 8 - Part 2: {}", output_digits(&data));
}

fn unique_output_digits(data: &Vec<String>) -> i32 {
    let mut result = 0;

    for line in data {
        let line = line.split('|').collect::<Vec<&str>>()[1];
        let words: Vec<&str> = line.split_whitespace().collect();

        for word in words {
            match word.len() {
                2 | 3 | 4 | 7 => result += 1,
                _ => continue
            };
        }
    }

    result
}

fn output_digits(data: &Vec<String>) -> i32 {
    let mut result = 0;

    for line in data {
        let left_line = line.split('|').collect::<Vec<&str>>()[0];
        let words: Vec<&str> = left_line.split_whitespace().collect();

        let char_map = set_char_map(&words);

        let right_line = line.split('|').collect::<Vec<&str>>()[1];
        let words: Vec<&str> = right_line.split_whitespace().collect();

        let mut numbers: Vec<i32> = vec![];
        for word in words {
            let number = match word.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                5 => {
                    let (mut is_two, mut is_three) = (true, true);

                    let mut two = String::new();
                    two.push(*char_map.get(&'a').unwrap());
                    two.push(*char_map.get(&'c').unwrap());
                    two.push(*char_map.get(&'d').unwrap());
                    two.push(*char_map.get(&'e').unwrap());
                    two.push(*char_map.get(&'g').unwrap());

                    let mut three = String::new();
                    three.push(*char_map.get(&'a').unwrap());
                    three.push(*char_map.get(&'c').unwrap());
                    three.push(*char_map.get(&'d').unwrap());
                    three.push(*char_map.get(&'f').unwrap());
                    three.push(*char_map.get(&'g').unwrap());

                    for c in word.chars() {
                        if !two.contains(c) {
                            is_two = false;
                        }

                        if !three.contains(c) {
                            is_three = false;
                        }
                    }

                    if is_two {
                        2
                    } else if is_three {
                        3
                    } else {
                        5
                    }
                },
                6 => {
                    let (mut is_zero, mut is_six) = (true, true);

                    let mut zero = String::new();
                    zero.push(*char_map.get(&'a').unwrap());
                    zero.push(*char_map.get(&'b').unwrap());
                    zero.push(*char_map.get(&'c').unwrap());
                    zero.push(*char_map.get(&'e').unwrap());
                    zero.push(*char_map.get(&'f').unwrap());
                    zero.push(*char_map.get(&'g').unwrap());

                    let mut six = String::new();
                    six.push(*char_map.get(&'a').unwrap());
                    six.push(*char_map.get(&'b').unwrap());
                    six.push(*char_map.get(&'d').unwrap());
                    six.push(*char_map.get(&'e').unwrap());
                    six.push(*char_map.get(&'f').unwrap());
                    six.push(*char_map.get(&'g').unwrap());

                    for c in word.chars() {
                        if !zero.contains(c) {
                            is_zero = false;
                        }

                        if !six.contains(c) {
                            is_six = false;
                        }
                    }

                    if is_zero {
                        0
                    } else if is_six {
                        6
                    } else {
                        9
                    }
                },
                7 => 8,
                _ => -1
            };
            numbers.push(number);
        }

        let ten: i32 = 10;
        let temp = (numbers[0] * ten.pow(3)) + (numbers[1] * ten.pow(2)) + (numbers[2] * ten.pow(1)) + numbers[3];
        result += temp;
    }

    result
}

fn set_char_map(words: &Vec<&str>) -> HashMap<char, char> {
    let mut char_map = HashMap::from([
        ('a', ' '),
        ('b', ' '),
        ('c', ' '),
        ('d', ' '),
        ('e', ' '),
        ('f', ' '),
        ('g', ' ')
    ]);

    let mut possible_c_f: Vec<char> = vec![];
    let mut possible_b_d: Vec<char> = vec![];

    let mut map_set = 0;
    while map_set < 7 {
        for word in words {
            if word.len() == 2 && possible_c_f.len() == 0 {
                for c in word.chars() {
                    possible_c_f.push(c);
                }
            } else if word.len() == 4 && possible_c_f.len() == 2 && possible_b_d.len() == 0 {
                for c in word.chars() {
                    if !possible_c_f.contains(&c) {
                        possible_b_d.push(c);
                    }
                }
            } else if word.len() == 3 && map_set == 0 && possible_c_f.len() == 2 {
                for c in word.chars() {
                    if !possible_c_f.contains(&c) {
                        char_map.insert('a', c);
                        map_set += 1;
                    }
                }
            } else if word.len() == 6 && map_set == 1 && possible_c_f.len() == 2 {
                let contender_1 = possible_c_f[0];
                let contender_2 = possible_c_f[1];

                if word.contains(contender_1) && !word.contains(contender_2) {
                    char_map.insert('c', contender_2);
                    char_map.insert('f', contender_1);
                    map_set += 2;
                } else if word.contains(contender_2) && !word.contains(contender_1) {
                    char_map.insert('c', contender_1);
                    char_map.insert('f', contender_2);
                    map_set += 2;
                }
            } else if word.len() == 6 && map_set == 3 && possible_b_d.len() == 2 {
                let contender_1 = possible_b_d[0];
                let contender_2 = possible_b_d[1];

                if word.contains(contender_1) && !word.contains(contender_2) {
                    char_map.insert('d', contender_2);
                    char_map.insert('b', contender_1);
                    map_set += 2;
                } else if word.contains(contender_2) && !word.contains(contender_1) {
                    char_map.insert('d', contender_1);
                    char_map.insert('b', contender_2);
                    map_set += 2;
                }
            } else if word.len() == 5 && map_set == 5 {
                let mut contains_all_chars = true;

                let mut available_chars = String::new();
                available_chars.push(*char_map.get(&'a').unwrap());
                available_chars.push(*char_map.get(&'b').unwrap());
                available_chars.push(*char_map.get(&'d').unwrap());
                available_chars.push(*char_map.get(&'f').unwrap());
                let available_chars: Vec<char> = available_chars.chars().collect();

                for c in &available_chars {
                    if !word.contains(*c) {
                        contains_all_chars = false;
                    }
                }

                if contains_all_chars {
                    for c in word.chars() {
                        if !available_chars.contains(&c) {
                            char_map.insert('g', c);
                            map_set += 1;
                            break;
                        }
                    }
                }
            } else if word.len() == 7 && map_set == 6 {
                let mut available_chars = String::new();
                available_chars.push(*char_map.get(&'a').unwrap());
                available_chars.push(*char_map.get(&'b').unwrap());
                available_chars.push(*char_map.get(&'c').unwrap());
                available_chars.push(*char_map.get(&'d').unwrap());
                available_chars.push(*char_map.get(&'f').unwrap());
                available_chars.push(*char_map.get(&'g').unwrap());
                let available_chars: Vec<char> = available_chars.chars().collect();
                let available_chars = String::from_iter(available_chars.into_iter());

                for c in word.chars() {
                    if !available_chars.contains(c) {
                        char_map.insert('e', c);
                        map_set += 1;
                    }
                }
            }
        }
    }
    
    char_map
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data: Vec<String> = vec![
            String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
            String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"),
            String::from("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"),
            String::from("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"),
            String::from("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"),
            String::from("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"),
            String::from("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"),
            String::from("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"),
            String::from("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"),
            String::from("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce")
        ];
        let expected = 26;
        let actual = super::unique_output_digits(&data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data: Vec<String> = vec![
            String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
            String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"),
            String::from("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"),
            String::from("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"),
            String::from("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"),
            String::from("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"),
            String::from("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"),
            String::from("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"),
            String::from("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"),
            String::from("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce")
        ];
        let expected = 61229;
        let actual = super::output_digits(&data);
        assert_eq!(expected, actual);
    }
}

