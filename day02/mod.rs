pub fn part_one(input: &str) -> usize {
    let mut result: usize = 0;
    for line in input.trim().split('\n') {
        let split: Vec<&str> = line.trim().split(' ').collect();
        let you:  Shape = Shape::from_string(split[0]);
        let me:   Shape = Shape::from_string(split[1]);

        result +=  me.get_shape_score() + me.get_match_score(you);

    }
    println!("day02 -> part one: {}", result);
    return result;
}

pub fn part_two(input: &str) -> usize {
    let mut result: usize = 0;
    for line in input.trim().split('\n') {
        let split: Vec<&str> = line.trim().split(' ').collect();
        let you:  Shape = Shape::from_string(split[0]);
        let me:   Shape = Shape::from_string(split[1]);

        result += me.get_match_score_2(you);

    }
    println!("day02 -> part two: {}", result);
    return result;
}

enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    // A Rock
    // B Paper
    // C Scissors
    fn from_string(shape_str: &str) -> Shape {
        match shape_str {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _   => panic!("unknown shape")
        }
    }

    // Rock     - 1P
    // Paper    - 2P
    // Scissors - 3P
    fn get_shape_score(&self) -> usize {
        match self {
            Shape::Rock     => 1,
            Shape::Paper    => 2,
            Shape::Scissors => 3,
        }
    }
    //  Loose - 0P
    //  Draw  - 3P
    //  Win   - 6P
    fn get_match_score(&self, against: Shape) -> usize {
        match self {
            Shape::Rock => {
                match against {
                    Shape::Scissors => 6,
                    Shape::Rock     => 3,
                    Shape::Paper    => 0,
                }
            }
            Shape::Paper => {
                match against {
                    Shape::Rock     => 6,
                    Shape::Paper    => 3,
                    Shape::Scissors => 0,
                }
            }
            Shape::Scissors => {
                match against {
                    Shape::Paper    => 6,
                    Shape::Scissors => 3,
                    Shape::Rock     => 0,
                }
            }
        }
    }
    // helper for part two
    // Rock     - Lose
    // Paper    - Draw
    // Scissors - Win
    fn get_match_score_2(&self, against: Shape) -> usize {
        const LOOSE_POINTS: usize = 0;
        const DRAW_POINTS: usize  = 3;
        const WIN_POINTS: usize   = 6;
        match self {
            // loose
            Shape::Rock => {
                LOOSE_POINTS + match against {
                    Shape::Scissors => Shape::Paper.get_shape_score(),
                    Shape::Rock     => Shape::Scissors.get_shape_score(),
                    Shape::Paper    => Shape::Rock.get_shape_score(),
                }
            }
            // draw
            Shape::Paper => {
                DRAW_POINTS + match against {
                    Shape::Rock     => Shape::Rock.get_shape_score(),
                    Shape::Paper    => Shape::Paper.get_shape_score(),
                    Shape::Scissors => Shape::Scissors.get_shape_score(),
                }
            }
            // win
            Shape::Scissors => {
                WIN_POINTS + match against {
                    Shape::Paper    => Shape::Scissors.get_shape_score(),
                    Shape::Scissors => Shape::Rock.get_shape_score(),
                    Shape::Rock     => Shape::Paper.get_shape_score(),
                }
            }
        }

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 15);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 12);
    }
}
