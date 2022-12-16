pub fn part_one(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = vec![];
    for monkey in input.trim().split("\n\n") {
        // TODO: parse monkeys from input
        // starting items
        // operation
        let description: Vec<&str> = monkey.split('\n').collect();

        let items: Vec<usize> = description[1].replace("Starting items:", "")
            .trim().to_string()
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect();

        // TODO: target operation can be 'old' ...
        let operations: Vec<String> = description[2].replace("Operation: new = old ", "")
            .trim().to_string().split(' ')
            .map(|x| x.to_string())
            .collect();

        let divider: usize = description[3].replace("Test: divisible by ", "").trim().to_string().parse::<usize>().unwrap();

        let target_true: usize = description[4].replace("If true: throw to monkey", "").trim().to_string().parse::<usize>().unwrap();
        let target_false: usize = description[5].replace("If false: throw to monkey", "").trim().to_string().parse::<usize>().unwrap();

    }
    for _ in 0..20 {
        // do monkey business
    }

    monkeys[0].add_item(3);
    monkeys[0].inspect_items();
    for item in monkeys[0].get_items() {
        monkeys[1].add_item(item.0);
    }

    let mut monkey_business: Vec<usize> = monkeys.iter().map(|monkey| monkey.inspections).collect();
    monkey_business.sort();

    let result: usize = monkey_business.pop().unwrap() * monkey_business.pop().unwrap();
    println!("{:?}", result);

    return result;
}

pub fn part_two(input: &str) -> usize {
    return 1;
}

#[derive (Debug)]
struct Monkey {
    items: Vec<usize>,
    operation_value: usize,
    operation_is_add: bool,
    inspections: usize,
    divider: usize,
    target_true: usize,
    target_false: usize
}

impl Monkey {
    // TODO: pass constructor arguments
    pub fn new(
        items: Vec<usize>, 
        operation_value: usize, 
        operation_is_add: bool, 
        divider: usize, 
        target_true: usize, 
        target_false: usize
        ) -> Self {

        return Self {
            items,
            operation_value,
            operation_is_add,
            inspections: 0,
            divider,
            target_true,
            target_false,
        }
    }
    pub fn inspect_items(&mut self) {
        self.inspections += self.items.len();
        self.items = self.items.iter()
            .map(|x| if self.operation_is_add {x + self.operation_value} else {x * self.operation_value}).collect();
    }
    pub fn get_items(&mut self) -> Vec<(usize, usize)> {
        return self.items.drain(..)
            .map(|item| (item, if item % self.divider == 0
                         {self.target_true} else {self.target_false})).collect();
    }
    pub fn add_item(&mut self, value: usize) {
        self.items.push(value);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 10605);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 1);
    }
}
