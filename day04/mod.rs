pub fn part_one(input: &str) -> usize {
    let mut result: usize = 0;

    for line in input.trim().split('\n') {
        let (pair_1, pair_2) = prepare_input(line);
        if is_contained(pair_1, pair_2) {
            result += 1;
        }
    }

    println!("day04 -> part one: {}", result);
    return result;
}
pub fn part_two(input: &str) -> usize {
    let mut result: usize = 0;

    for line in input.trim().split('\n') {
        let (pair_1, pair_2) = prepare_input(line);
        if has_overlap(pair_1, pair_2) {
            result += 1;
        }
    }

    println!("day04 -> part two: {}", result);
    return result;
}

pub fn prepare_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let pairs: Vec<&str> = input.trim().split(',').collect();
    let pair_1: Vec<i32> = pairs[0].split('-').map(|x| x.parse::<i32>().unwrap()).collect();
    let pair_2: Vec<i32> = pairs[1].split('-').map(|x| x.parse::<i32>().unwrap()).collect();

    return (pair_1, pair_2)
}

pub fn is_contained(first: Vec<i32>, second: Vec<i32>) -> bool {
    if first.len() != 2 || second.len() != 2 {
        panic!("excpected vec of lenght 2!");
    }
    return first[0] >= second[0] && first[1] <= second[1] ||
           second[0] >= first[0] && second[1] <= first[1];
}

pub fn has_overlap(first: Vec<i32>, second: Vec<i32>) -> bool {
    if first.len() != 2 || second.len() != 2 {
        panic!("excpected vec of lenght 2!");
    }
    return first[1] >= second[0] && first[0] <= second[0] ||
        second[1] >= first[0] && second[0] <= first[0];
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 2);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 4);
    }
}
