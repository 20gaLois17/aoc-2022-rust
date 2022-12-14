pub fn part_one(input: &str) -> i32 {
    let mut result: i32 = 0;
    let mut processor = Processor::new();
    for line in input.trim().split('\n') {
        // each line is an instruction
        let op: Vec<&str> = line.split(' ').collect();
        match op[0] {
            "noop" => result += processor.nop(),
            "addx" => result += processor.addx(op[1].parse::<i32>().unwrap()),
            _ => panic!("unknown command")
        }
    }
    println!("day10 -> part one: {}", result);
    return result;
}

pub fn part_two(input: &str) -> i32 {
    let mut processor = Processor::new();
    let mut _result: i32 = 0;
    for line in input.trim().split('\n') {
        // each line is an instruction
        let op: Vec<&str> = line.split(' ').collect();
        match op[0] {
            "noop" => _result += processor.nop(),
            "addx" => _result += processor.addx(op[1].parse::<i32>().unwrap()),
            _ => panic!("unknown command")
        }
    }
    return -1;
}

#[derive (Debug)]
struct Processor {
    cycle: i32,
    x: i32,
}

impl Processor {
    fn new() -> Processor {
        Processor {
            cycle: 0,
            x: 1
        }
    }
    fn addx(&mut self, val: i32) -> i32 {
        let mut result: i32 = 0;
        for _ in 0..2 {
            self.cycle += 1;
            self.draw_pixel();
            result += self.get_signal_strength();
        }
        self.x += val;
        return result;
    }
    fn nop(&mut self) -> i32 {
        let mut result: i32 = 0;
        for _ in 0..1 {
            self.cycle += 1;
            self.draw_pixel();
            result += self.get_signal_strength();
        }
        return result;
    }
    fn get_signal_strength(&self) -> i32 {
        if self.cycle % 40 == 20 && self.cycle <= 220 {
            return self.x * self.cycle;
        } else {
            return 0;
        }
    }
    fn draw_pixel(&self) {
        if ((self.cycle-1)%40 - self.x).abs() < 2 {
            print!("#");
        } else {
            print!(".");
        }
        if self.cycle % 40 == 0 {
            print!("\n");
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 13140);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 1);
    }
}
