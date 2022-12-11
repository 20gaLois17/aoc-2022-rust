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
    return 1;
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
        assert_eq!(super::part_two(include_str!("testinput")), 0);
    }
}
