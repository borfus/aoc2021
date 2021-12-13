use std::fs;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub fn run() {
    let lines = fs::read_to_string("input/day12.txt").unwrap();
    let lines = lines.lines().collect();

    println!("Day 12 - Part 1: {}", count_small_cave_paths(&lines, false));
    println!("Day 12 - Part 2: {}", count_small_cave_paths(&lines, true));
}

fn count_small_cave_paths(data: &Vec<&str>, more_than_once: bool) -> i32 {
    let mut cave_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in data {
        let nodes: Vec<&str> = line.split('-').collect();
        let n1 = nodes[0];
        let n2 = nodes[1];

        cave_map.entry(String::from(n1)).or_default().push(String::from(n2));
        cave_map.entry(String::from(n2)).or_default().push(String::from(n1));
    }

    let visited: Rc<RefCell<HashMap<String, i32>>> = Rc::new(RefCell::new(HashMap::new()));

    search(String::from("start"), visited, &cave_map, 0, more_than_once)
}

fn search(cave: String, visited: Rc<RefCell<HashMap<String, i32>>>, cave_map: &HashMap<String, Vec<String>>, mut count: i32, more_than_once: bool) -> i32 {
    if cave == "end" {
        return count + 1;
    }

    if cave.chars().nth(0).unwrap().is_lowercase() {
        *visited.borrow_mut().entry(cave.clone()).or_default() += 1;

        let mut visited_twice = 0;

        let map_clone = visited.borrow().clone();
        for (_, v) in map_clone.iter() {
            if more_than_once {
                if *v > 1 {
                    visited_twice += 1;
                }

                if *v > 2 {
                    *visited.borrow_mut().entry(cave.clone()).or_default() -= 1;
                    return count;
                }
            } else {
                if *v > 1 {
                    *visited.borrow_mut().entry(cave.clone()).or_default() -= 1;
                    return count;
                }
            }
        }

        if more_than_once {
            if visited_twice > 1 {
                *visited.borrow_mut().entry(cave.clone()).or_default() -= 1;
                return count;
            }
        }
    }

    for neighbor in cave_map.get(&cave).unwrap() {
        if neighbor == "start" {
            continue;
        }

        count = search(String::from(neighbor), Rc::clone(&visited), cave_map, count, more_than_once);
    }

    if cave.chars().nth(0).unwrap().is_lowercase() {
        *visited.borrow_mut().entry(cave).or_default() -= 1;
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data: Vec<&str> = vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end"
        ];
        let expected = 10;
        let actual = super::count_small_cave_paths(&data, false);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data: Vec<&str> = vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end"
        ];
        let expected = 36;
        let actual = super::count_small_cave_paths(&data, true);
        assert_eq!(expected, actual);
    }
}

