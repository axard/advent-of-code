fn main() {
    let input = include_bytes!("../../res/aoc2015/1/input.txt");
    let input = String::from_utf8_lossy(input);

    let decoder = |c: char| -> i32 {
        if c.eq(&'(') {
            1
        } else if c.eq(&')') {
            -1
        } else {
            0
        }
    };

    let floor: i32 = input.chars().map(decoder).sum();

    println!("part 1: {} floor", floor);

    let mut floor: i32 = 0;
    let mut number: usize = 0;

    for (i, v) in input.chars().map(decoder).enumerate() {
        floor += v;
        if floor == -1 {
            number = i + 1;
            break;
        }
    }

    println!("part 2: on {} command number", number)
}
