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
pub fn part_two(input: &str) -> usize {
    let totals: Vec<usize> = input.trim()
        .split("\n\n")
        .map(|x| x.split("\n").map(|y| y.parse::<usize>().unwrap()).sum())
        .collect();

    let result: usize = sum_of_largest_n(totals, 3);

    println!("day01 -> part two: {}", result);
    return result;
}

pub fn sum_of_largest_n(mut list: Vec<usize>, n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    for _ in 0..n {
        // bubble sort for the first n integers,
        // largest n will move to the end of the end
        for j in 0..list.len()-1 {
            if list[j] > list[j+1] {
                let tmp: usize = list[j];
                list[j] = list[j+1];
                list[j+1] = tmp;
            }
        }
    }
    let mut sum: usize = 0;
    // sum up largest n integers
    for i in 0..n {
        sum += list[list.len() - (1 + i)];
    }
    return sum;
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_of_largest_n() {
        assert_eq!(super::sum_of_largest_n(vec![5, 3, 4, 2], 2), 9);
        assert_eq!(super::sum_of_largest_n(vec![2, 2, 1, 0], 3), 5);
    }
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 24000);
    }
    #[test]
    fn part_two() {
            assert_eq!(super::part_two(include_str!("testinput")), 45000);
    }
}
