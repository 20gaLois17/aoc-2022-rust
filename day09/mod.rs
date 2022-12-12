use std::collections::HashSet;
pub fn part_one(input: &str) -> usize {
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    let mut rope: Vec<Coordinate> = vec![];
    for _ in 0..2 {
        rope.push(Coordinate { x: 0, y: 0 });
    }

    for line in input.trim().split('\n') {
        let tmp: Vec<&str> = line.split(' ').collect();
        let num = tmp[1].parse::<usize>().unwrap();
        for _ in 0..num {
            rope[0].mv(tmp[0]);
            let coor: Coordinate = rope[0].clone();
            if rope[1].follow(coor) {
                tail_positions.insert((rope[1].x, rope[1].y));
            }
        }
    }
    let result = tail_positions.len();
    println!("day09 -> part one: {}", result);
    return result;
}

pub fn part_two(input: &str) -> usize {
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    let mut rope: Vec<Coordinate> = vec![];
    for _ in 0..10 {
        rope.push(Coordinate { x: 0, y: 0 });
    }

    for line in input.trim().split('\n') {
        let tmp: Vec<&str> = line.split(' ').collect();
        let num = tmp[1].parse::<usize>().unwrap();
        for _ in 0..num {
            rope[0].mv(tmp[0]);
            for i in 1..rope.len() {
                let coor: Coordinate = rope[i-1].clone();
                if i == rope.len()-1 {
                    if rope[i].follow(coor) {
                        tail_positions.insert((rope[i].x, rope[i].y));
                    }
                } else {
                    rope[i].follow(coor);
                }
            }
        }
    }

    let result = tail_positions.len();
    println!("day09 -> part two: {}", result);
    return result;
}

#[derive (Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn mv(&mut self, dir: &str) {
        match dir {
            "R" => self.x += 1,
            "L" => self.x -= 1,
            "U" => self.y += 1,
            "D" => self.y -= 1,
             _  => panic!("cannot parse dir"),
        }
    }

    // return true if move occured, false if not
    fn follow(&mut self, target: Coordinate) -> bool {
        let delta_x: i32 = self.x - target.x;
        let delta_y: i32 = self.y - target.y;

        // head and tail are on the same row or in the same column
        if delta_x == 0 || delta_y == 0 {
            if delta_x.abs() == 2 {
                self.x -= delta_x/2;
                return true;
            }
            if delta_y.abs() == 2 {
               self.y -= delta_y/2;
               return true;
            }
        }
        // head and tail are not on the same row nor column
        else {
            // this case can only occur in part two
            if delta_x.abs() == 2 && delta_y.abs() == 2 {
                self.x -= delta_x/2;
                self.y -= delta_y/2;
                return true;
            }
            if delta_x.abs() == 2 {
                self.x -= delta_x/2;
                self.y -= delta_y;
                return true;
            }
            if delta_y.abs() == 2 {
                self.y -= delta_y/2;
                self.x -= delta_x;
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 13);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 1);
        assert_eq!(super::part_two(include_str!("testinput2")), 36);
    }
}
