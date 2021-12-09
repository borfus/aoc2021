use std::fs;

pub fn run() {
    let lines = fs::read_to_string("input/day9.txt").unwrap();
    let lines = lines.lines();

    let mut data: Vec<String> = vec![];
    for line in lines {
        data.push(String::from(line));
    }

    println!("Day 8 - Part 1: {}", height_map_sum(&data, false));
    println!("Day 8 - Part 2: {}", height_map_sum(&data, true));
}

fn height_map_sum(data: &Vec<String>, find_basin: bool) -> i32 {
    let mut result = 0;
    let mut height_map: Vec<Vec<i32>> = vec![];
    let mut basin_totals: Vec<i32> = vec![];

    for line in data {
        let mut row: Vec<i32> = vec![];
        for c in line.chars() {
            row.push(c.to_string().parse().unwrap());
        }
        height_map.push(row);
    }

    for x in 0..height_map.len() {
        for y in 0..height_map[x].len() {
            if is_low_point(height_map[x][y], &height_map, x, y) {
                if !find_basin {
                    result += height_map[x][y] + 1;
                } else {
                    basin_totals.push(get_basin_sum(&height_map, x, y));
                }
            }
        }
    }

    if find_basin {
        basin_totals.sort();
        let len = basin_totals.len();
        result = basin_totals[len-1] * basin_totals[len-2] * basin_totals[len-3];
    }

    result
}

fn is_low_point(current_height: i32, height_map: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let mut neighbors: Vec<i32> = vec![];

    if x != 0 {
        neighbors.push(height_map[x-1][y]);
    }

    if x != height_map.len() - 1 {
        neighbors.push(height_map[x+1][y]);
    }

    if y != 0 {
        neighbors.push(height_map[x][y-1]);
    }

    if y != height_map[x].len() - 1 {
        neighbors.push(height_map[x][y+1]);
    }

    for n in neighbors {
        if current_height >= n {
            return false;
        }
    }

    true
}

fn get_basin_sum(height_map: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut added_coords: Vec<(usize, usize)> = vec![];
    added_coords.push((x, y));

    let mut i = 0;
    while i < added_coords.len() {
        let x = added_coords[i].0;
        let y = added_coords[i].1;
        if x != 0 {
            let coords = (x-1, y);
            if !added_coords.contains(&coords) && height_map[coords.0][coords.1] != 9 {
                added_coords.push(coords);
            }
        }

        if x != height_map.len() - 1 {
            let coords = (x+1, y);
            if !added_coords.contains(&coords) && height_map[coords.0][coords.1] != 9 {
                added_coords.push(coords);
            }
        }

        if y != 0 {
            let coords = (x, y-1);
            if !added_coords.contains(&coords) && height_map[coords.0][coords.1] != 9 {
                added_coords.push(coords);
            }
        }

        if y != height_map[x].len() - 1 {
            let coords = (x, y+1);
            if !added_coords.contains(&coords) && height_map[coords.0][coords.1] != 9 {
                added_coords.push(coords);
            }
        }
        i += 1;
    }

    added_coords.len() as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let data: Vec<String> = vec![
            String::from("2199943210"),
            String::from("3987894921"),
            String::from("9856789892"),
            String::from("8767896789"),
            String::from("9899965678")
        ];
        let expected = 15;
        let actual = super::height_map_sum(&data, false);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2() {
        let data: Vec<String> = vec![
            String::from("2199943210"),
            String::from("3987894921"),
            String::from("9856789892"),
            String::from("8767896789"),
            String::from("9899965678")
        ];
        let expected = 1134;
        let actual = super::height_map_sum(&data, true);
        assert_eq!(expected, actual);
    }
}

