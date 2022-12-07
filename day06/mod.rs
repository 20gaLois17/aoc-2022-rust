pub fn part_one(input: &str) -> usize {
    const SIGNAL_LENGTH: usize = 4;
    let sequence: Vec<char> = input.trim().chars().collect();
    let result = get_index_of_first_n_distinct(sequence, SIGNAL_LENGTH);

    println!("day04 -> part one: {}", result);
    return result;
}

pub fn part_two(input: &str) -> usize {
    const SIGNAL_LENGTH: usize = 14;
    let sequence: Vec<char> = input.trim().chars().collect();
    let result = get_index_of_first_n_distinct(sequence, SIGNAL_LENGTH);

    println!("day04 -> part two: {}", result);
    return result;
}

pub fn get_index_of_first_n_distinct(message: Vec<char>, len: usize) -> usize {
    for i in len-1..message.len() {
        for j in 0..len {
            if message[i-(len-1)..i-j].contains(&message[i-j]) {
                break;
            }
            if j == len-1 {
                return i + 1; // 'index' in this case starting with 1
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(super::part_one("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(super::part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }
}
