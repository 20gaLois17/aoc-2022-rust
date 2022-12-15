pub fn part_one(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = vec![];
    for monkey in input.trim().split("\n\n") {
        // TODO: parse monkeys from input
        monkeys.push(Monkey::new());
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
    pub fn new() -> Self {
        return Self {
            items: vec![1, 2],
            operation_value: 5,
            operation_is_add: true,
            inspections: 0,
            divider: 3,
            target_true: 9,
            target_false: 11
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
