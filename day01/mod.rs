pub fn part_one(input: &str) -> usize {
    let mut max: usize = 0;
    for chunk in input.trim().split("\n\n") {
        let sum: usize = chunk.trim()
            .split("\n")
            .map(|x| x.parse::<usize>().unwrap()).sum();
        if max < sum {
            max = sum;
        }
    }
    println!("day01 -> part one: {}", max);
    return max;
}
pub fn part_two(input: &str) -> i64 {
    return -1;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 24000);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 1);
    }
}
