use std::fs;
use to_binary::BinaryString;

pub fn run() {
    let mut data = fs::read_to_string("input/day16.txt").unwrap();
    data.pop();

    println!("Day 16 - Part 1: {}", parse_packets(&data.as_str(), true));
    println!("Day 16 - Part 2: {}", parse_packets(&data.as_str(), false));
}

fn parse_packets(data: &str, sum_ids: bool) -> i32 {
    let mut result = 0;
    let binary = BinaryString::from_hex(data).unwrap();
    let bin_str = &binary.0;
    let binary_len = binary.bits().unwrap();

    let mut i = 0;
    while i < binary_len {
        let remaining_zeroes: String = vec!['0'; binary_len - i].iter().collect::<String>();
        if bin_str == &remaining_zeroes || binary_len - i < 11 {
            break;
        }

        let id = bin_to_dec(&bin_str[i..i+3]);
        if sum_ids {
            result += id;
        }

        let type_id = bin_to_dec(&bin_str[i+3..i+6]);

        if type_id == 4 {
            if !sum_ids {
                result += eval_literal_packet(&bin_str[i..]);
            }
            i += literal_packet_len(&bin_str[i..]);
        } else {
            if !sum_ids {
                let (val, next_i) = eval_operator_packet(&bin_str[i..], type_id);
                result += val;
                i += next_i;
            } else {
                i += operator_packet_len(&bin_str[i..]);
            }
        }
    }

    result
}

// fn parse_packet(bin_str: &str) -> i32 {
//     let mut result = 0;

//     let type_id = bin_to_dec(&bin_str[3..6]);

//     if type_id == 4 {
//         result += eval_literal_packet(&bin_str);
//     } else {
//         let (val, _) = eval_operator_packet(&bin_str, type_id);
//         result += val;
//     }

//     result
// }

fn literal_packet_len(bin_str: &str) -> usize {
    let bin_chars: Vec<char> = bin_str.chars().collect();

    let mut i = 6;
    while i < bin_str.len() {
        if bin_chars[i] == '1' {
            i += 5;
            continue;
        } else {
            i += 5;
            break;
        }
    }

    i
}

fn operator_packet_len(bin_str: &str) -> usize {
    let bin_chars: Vec<char> = bin_str.chars().collect();

    let mut i = 6;
    if bin_chars[i] == '0' {
        i += 16;
    } else {
        i += 12;
    }

    i
}

fn bin_to_dec(bin_str: &str) -> i32 {
    i32::from_str_radix(&bin_str, 2).unwrap()
}

fn eval_literal_packet(bin_str: &str) -> i32 {
    let mut result = 0;
    let bin_chars: Vec<char> = bin_str.chars().collect();

    let mut i = 6;
    loop {
        if bin_chars[i] == '0' {
            i += 1;
            result += bin_to_dec(&bin_str[i..i+4]);
            break;
        } else {
            i += 1;
            result += bin_to_dec(&bin_str[i..i+4]);
            i += 4;
        }
    }

    result
}

fn eval_operator_packet(bin_str: &str, type_id: i32) -> (i32, usize) {
    let mut result = 0;

    let bin_chars: Vec<char> = bin_str.chars().collect();

    let packet_len;
    let mut values: Vec<i32> = vec![];
    let mut i = 6;
    if bin_chars[i] == '0' {
        i += 1;
        packet_len = bin_to_dec(&bin_str[i..i+15]) as usize;
        i += 15;

        let start_i = i;
        let mut next_i = i;
        loop {
            let next_packet_type_id = bin_to_dec(&bin_str[next_i+3..next_i+6]);
            if next_packet_type_id == 4 {
                next_i += literal_packet_len(&bin_str[next_i..]);
                result += eval_literal_packet(&bin_str[i..]);
                return (result, next_i);
            } else {
                next_i += operator_packet_len(&bin_str[next_i..]);
                let (val, new_i) = eval_operator_packet(&bin_str[next_i..], next_packet_type_id);
                values.push(val);
                next_i = new_i;
            }

            i = next_i;

            if next_i - start_i == packet_len {
                break;
            }
        }
    } else {
        i += 1;
        packet_len = bin_to_dec(&bin_str[i..i+11]) as usize;
        i += 11;

        let mut next_i = i;
        for _ in 0..packet_len {
            let next_packet_type_id = bin_to_dec(&bin_str[next_i+3..next_i+6]);

            if next_packet_type_id == 4 {
                next_i += literal_packet_len(&bin_str[next_i..]);
                result += eval_literal_packet(&bin_str[i..]);
                return (result, next_i);
            } else {
                next_i += operator_packet_len(&bin_str[next_i..]);
                let (val, new_i) = eval_operator_packet(&bin_str[next_i..], next_packet_type_id);
                values.push(val);
                next_i = new_i;
            }

            i = next_i;
        }
    }

    match type_id {
        0 => {
            for value in values {
                result += value;
            }
        },
        1 => {
            if values.len() == 1 {
                result += values[0];
            } else {
                for value in values {
                    if result == 0 {
                        result += value;
                    } else {
                        result *= value;
                    }
                }
            }
        },
        2 => {
            let mut min = i32::MAX;
            for value in values {
                if value < min {
                    min = value;
                }
            }
            result = min;
        },
        3 => {
            let mut max = i32::MIN;
            for value in values {
                if value > max {
                    max = value;
                }
            }
            result = max;
        },
        5 => {
            if values[0] > values[1] {
                result = 1;
            } else {
                result = 0;
            }
        },
        6 => {
            if values[0] < values[1] {
                result = 1;
            } else {
                result = 0;
            }
        },
        7 => {
            if values[0] == values[1] {
                result = 1;
            } else {
                result = 0;
            }
        },
        _ => println!("Invalid packet type id!")
    };

    (result, i)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data = "8A004A801A8002F478";
        let expected = 16;
        let actual = super::parse_packets(&data, true);
        assert_eq!(expected, actual);
        let data = "620080001611562C8802118E34";
        let expected = 12;
        let actual = super::parse_packets(&data, true);
        assert_eq!(expected, actual);
        let data = "C0015000016115A2E0802F182340";
        let expected = 23;
        let actual = super::parse_packets(&data, true);
        assert_eq!(expected, actual);
        let data = "A0016C880162017C3686B18A3D4780";
        let expected = 31;
        let actual = super::parse_packets(&data, true);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data = "C200B40A82";
        let expected = 3;
        let actual = super::parse_packets(&data, false);
        assert_eq!(expected, actual);
        let data = "04005AC33890";
        let expected = 54;
        let actual = super::parse_packets(&data, false);
        assert_eq!(expected, actual);
        let data = "880086C3E88112";
        let expected = 7;
        let actual = super::parse_packets(&data, false);
        assert_eq!(expected, actual);
        let data = "CE00C43D881120";
        let expected = 9;
        let actual = super::parse_packets(&data, false);
        assert_eq!(expected, actual);
        let data = "D8005AC2A8F0";
        let expected = 1;
        let actual = super::parse_packets(&data, false);
        assert_eq!(expected, actual);
        let data = "F600BC2D8F";
        let expected = 0;
        let actual = super::parse_packets(&data, false);
        assert_eq!(expected, actual);
        let data = "9C005AC2F8F0";
        let expected = 0;
        let actual = super::parse_packets(&data, false);
        assert_eq!(expected, actual);
        let data = "9C0141080250320F1802104A08";
        let expected = 1;
        let actual = super::parse_packets(&data, false);
        assert_eq!(expected, actual);
    }
}

