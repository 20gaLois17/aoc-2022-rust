use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    let mut result: usize = 0;
    for line in input.trim().split('\n') {
        let chars: Vec<char> = line.chars().collect();
        let compartment_size:usize = chars.len()/2;
        let mut first_compartment: HashSet<char> = HashSet::new();
        let mut second_compartment: HashSet<char> = HashSet::new();
        for i in 0..chars.len() {
            if i < compartment_size {
                first_compartment.insert(chars[i]);
            }
            else {
                second_compartment.insert(chars[i]);
            }
        }
        // we need to assume from the problem description, that this intersection will always
        // contain exactly one char
        result += get_type_score(*first_compartment.intersection(&second_compartment).next().unwrap());
    }
    println!("day03 -> part one: {}", result);
    return result;
}
pub fn part_two(input: &str) -> usize {
    let mut result: usize = 0;
    let lines_array: Vec<&str> = input.trim().split('\n').collect();
    let mut index: usize = 0;
    while index + 2 < lines_array.len() {
        let mut backpacks: Vec<HashSet<char>> = vec![HashSet::new(), HashSet::new(), HashSet::new()];
        // hydrate all the backpacks
        for i in 0..3 {
            for c in lines_array[index + i].chars() {
                backpacks[i].insert(c);
            }
        }
        let first_intersection: HashSet<_> = backpacks[0].intersection(&backpacks[1]).collect();
        let second_intersection: HashSet<_> = backpacks[1].intersection(&backpacks[2]).collect();

        // we need to assume from the problem description, that this intersection will always
        // contain exactly one char
        result += get_type_score(**first_intersection.intersection(&second_intersection).next().unwrap());
        index += 3;
    }

    println!("day03 -> part one: {}", result);
    return result;
}

pub fn get_type_score(c: char) -> usize {
    let num = c as usize;
    // we subtract some ascii offsets to achieve actual score like intended in problem
    if num > 96 {
        // lower case letter
        return num - '`' as usize;
    } else {
        // upper case letter
        return num - '@' as usize + 26; // add static offset of 26 to upper case letters
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn get_type_score() {
        assert_eq!(super::get_type_score('a'), 1);
        assert_eq!(super::get_type_score('p'), 16);
        assert_eq!(super::get_type_score('L'), 38);
    }
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 157);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 70);
    }
}
