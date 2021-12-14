use std::fs;
use std::collections::HashMap;

pub fn run() {
    let lines = fs::read_to_string("input/day14.txt").unwrap();
    let lines = lines.lines().collect();

    println!("Day 14 - Part 1: {}", find_range(&lines, 10));
    println!("Day 14 - Part 2: {}", find_range(&lines, 40));
}

fn find_range(data: &Vec<&str>, steps: i32) -> u128 {
    let mut polymer_template: HashMap<String, u128> = HashMap::new();
    let mut rules: HashMap<String, char> = HashMap::new();

    for i in 0..data[0].len()-1 {
        let mut pair = String::new();
        pair.push(data[0].chars().nth(i).unwrap());
        pair.push(data[0].chars().nth(i+1).unwrap());
        *polymer_template.entry(pair).or_default() += 1;
    }

    for i in 2..data.len() {
        let r: Vec<&str> = data[i].split(" -> ").collect();
        let from = r[0].chars().collect();
        let to = r[1].chars().nth(0).unwrap();
        rules.insert(from, to);
    }

    for _ in 0..steps {
        let mut new_polymer: HashMap<String, u128> = HashMap::new();

        for (k, v) in polymer_template {
            if rules.contains_key(&k) {
                let (mut e1, mut e2) = (String::new(), String::new());
                e1.push(k.chars().nth(0).unwrap());
                e1.push(*rules.get(&k).unwrap());

                e2.push(*rules.get(&k).unwrap());
                e2.push(k.chars().nth(1).unwrap());

                *new_polymer.entry(e1).or_default() += v;
                *new_polymer.entry(e2).or_default() += v;
            }
        }

        polymer_template = new_polymer;
    }

    let mut char_map: HashMap<char, u128> = HashMap::new();
    for (k, v) in &polymer_template {
        *char_map.entry(k.chars().nth(0).unwrap()).or_default() += v;
    }
    *char_map.entry(data[0].chars().nth(data[0].len()-1).unwrap()).or_default() += 1;

    let mut most_common = 0;
    let mut least_common = u128::MAX;

    for (_, v) in &char_map {
        if *v > most_common {
            most_common = *v;
        }

        if *v < least_common {
            least_common = *v;
        }
    }

    most_common - least_common
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data: Vec<&str> = vec![
            "NNCB",
            "",
            "CH -> B",
            "HH -> N",
            "CB -> H",
            "NH -> C",
            "HB -> C",
            "HC -> B",
            "HN -> C",
            "NN -> C",
            "BH -> H",
            "NC -> B",
            "NB -> B",
            "BN -> B",
            "BB -> N",
            "BC -> B",
            "CC -> N",
            "CN -> C"
        ];
        let expected = 1588;
        let actual = super::find_range(&data, 10);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data: Vec<&str> = vec![
            "NNCB",
            "",
            "CH -> B",
            "HH -> N",
            "CB -> H",
            "NH -> C",
            "HB -> C",
            "HC -> B",
            "HN -> C",
            "NN -> C",
            "BH -> H",
            "NC -> B",
            "NB -> B",
            "BN -> B",
            "BB -> N",
            "BC -> B",
            "CC -> N",
            "CN -> C"
        ];
        let expected = 2188189693529;
        let actual = super::find_range(&data, 40);
        assert_eq!(expected, actual);
    }
}

