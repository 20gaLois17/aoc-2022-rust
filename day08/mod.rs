use std::collections::HashSet;
pub fn part_one(input: &str) -> usize {
    let map = parse_trees(input);
    let height = map.len();
    let width = map[0].len();

    let mut counted_trees: HashSet<(usize, usize)> = HashSet::new();
    // count visible trees from 'left to right' and reverse
    for i in 0..height {
        let mut max_tree: i32 = -1;
        for j in 0..width {
            let current_tree = map[i][j];
            if current_tree > max_tree {
                max_tree = current_tree;
                counted_trees.insert((i, j));
            }
        }
        max_tree = -1;
        for j in (0..width).rev() {
            let current_tree = map[i][j];
            if current_tree > max_tree {
                max_tree = current_tree;
                counted_trees.insert((i, j));
            }
        }
    }
    // count visible trees from 'top to bottom' and reverse
    for j in 0..width {
        let mut max_tree: i32 = -1;
        for i in 0..height {
            let current_tree = map[i][j];
            if current_tree > max_tree {
                max_tree = current_tree;
                counted_trees.insert((i, j));
            }
        }
        max_tree = -1;
        for i in (0..height).rev() {
            let current_tree = map[i][j];
            if current_tree > max_tree {
                max_tree = current_tree;
                counted_trees.insert((i, j));
            }
        }
    }
    let result: usize = counted_trees.len();
    println!("day08 -> part one: {}", result);
    return result;
}

pub fn part_two(input: &str) -> usize {
    let map = parse_trees(input);
    let height = map.len();
    let width = map[0].len();
    let mut max_scenic_score: usize = 0;

    for i in 0..height {
        for j in 0..width {
            let local_scenic_score = 
                  peek(&map, (i, j), &'r')
                * peek(&map, (i, j), &'l')
                * peek(&map, (i, j), &'d')
                * peek(&map, (i, j), &'u');
            if local_scenic_score > max_scenic_score {
                max_scenic_score = local_scenic_score;
            }

        }
    }
    println!("day08 -> part two: {}", max_scenic_score);
    return max_scenic_score;
}

pub fn peek(map: &Vec<Vec<i32>>, pos: (usize, usize), dir: &char) -> usize {
    let mut result: usize = 0;
    let height = map.len();
    let width = map[0].len();
    match dir {
        'r' => {
            for i in pos.1+1..width {
                result += 1;
                if map[pos.0][i] >= map[pos.0][pos.1] {
                    break;
                }
            }
        },
        'l' => {
            for i in (0..pos.1).rev() {
                result += 1;
                if map[pos.0][i] >= map[pos.0][pos.1] {
                    break;
                }
            }
        },
        'd' => {
            for i in pos.0+1..height {
                result += 1;
                if map[i][pos.1] >= map[pos.0][pos.1] {
                    break;
                }
            }
        },
        'u' => {
            for i in (0..pos.0).rev() {
                result += 1;
                if map[i][pos.1] >= map[pos.0][pos.1] {
                    break;
                }
            }
        },
        _ => panic!("cannot match dir")
    }
    return result;
}

pub fn parse_trees(input: &str) -> Vec<Vec<i32>> {
    return input.trim().split('\n')
        .map(|line| line.chars()
             .map(|c| c as i32)
             .collect::<Vec<i32>>())
        .collect();
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 21);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 8);
    }
}
