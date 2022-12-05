pub fn part_one(input: &str) -> String {
    let mut result: String;
    let parts: Vec<&str> = input.split("\n\n").collect();
    let lines: Vec<&str> = parts[0].split('\n').collect();
    let commands: Vec<&str> = parts[1].trim().split('\n').collect();

    // parsing input as it is for some fun times ...
    let mut stacks: Vec<Vec<char>> = parse_stacks(lines);
    for command in commands {
        let chunks: Vec<&str> = command.split(' ').collect();

        // parse instruction and mutate stacks accordingly
        // ["move", "1", "from", "2", "to", "1"]
        let mut cnt: usize = chunks[1].parse::<usize>().unwrap();
        let source_no: usize = chunks[3].parse::<usize>().unwrap();
        let sink_no: usize = chunks[5].parse::<usize>().unwrap();
        while cnt > 0 {
            let tmp: char = stacks[source_no-1].pop().unwrap();
            stacks[sink_no-1].push(tmp);
            cnt -= 1;
        }
    }

    result = parse_result_from_stacks(stacks);
    println!("day04 -> part one: {}", result);
    return result;
}

pub fn part_two(input: &str) -> &str {
    let mut result: &str = &"B";

    println!("day04 -> part two: {}", result);
    return result;
}

pub fn parse_stacks(lines: Vec<&str>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![];
    for (idx, c) in lines[lines.len()-1].chars().enumerate() {
        if c != ' ' {
            let mut i: usize = lines.len()-1;
            let mut new_stack: Vec<char> = vec![];
            while i > 0 {
                let mut current_char: char = lines[i-1].chars().nth(idx).unwrap();
                if current_char != ' ' {
                    new_stack.push(current_char);
                }
                i -= 1;
            }
            stacks.push(new_stack);
        }
    }
    return stacks;
}

pub fn parse_result_from_stacks(stacks: Vec<Vec<char>>) -> String {
    let mut result: Vec<String> = vec![];
    for stack in stacks {
        result.push(stack.last().unwrap().to_string());
    }
    return result.join("");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), "CMZ".to_string());
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), "X");
    }
}
