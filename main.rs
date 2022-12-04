mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    day01::part_one(include_str!("./day01/input"));
    day01::part_one(include_str!("./day01/input"));

    day02::part_one(include_str!("./day02/input"));
    day02::part_two(include_str!("./day02/input"));

    day03::part_one(include_str!("./day03/input"));
    day03::part_two(include_str!("./day03/input"));

    day04::part_one(include_str!("./day04/input"));
    day04::part_two(include_str!("./day04/input"));
}
