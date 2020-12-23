use std::cmp::max;

fn main() {
    let input = include_bytes!("../../res/aoc2015/2/input.txt");
    let input = String::from_utf8_lossy(input);

    let mut full_area: i32 = 0;
    let mut full_length: i32 = 0;
    for line in input.lines() {
        let values: Vec<&str> = line.split('x').collect();
        if values.len() == 3 {
            let l = values[0].parse::<i32>().unwrap();
            let w = values[1].parse::<i32>().unwrap();
            let h = values[2].parse::<i32>().unwrap();

            full_area += area(l, w, h);
            full_length += length(l, w, h);
        }
    }

    println!("part 1: {} square feets", full_area);
    println!("part 2: {} feets", full_length);
}

mod test {
    #[test]
    fn test_area_2l_3w_4h() {
        assert_eq!(super::area(2, 3, 4), 58);
    }

    #[test]
    fn test_area_1l_1w_10h() {
        assert_eq!(super::area(1, 1, 10), 43);
    }
}

fn area(l: i32, w: i32, h: i32) -> i32 {
    fn min_area(l: i32, w: i32, h: i32) -> i32 {
        if max(l, max(w, h)) == h {
            l * w
        } else if max(l, max(w, h)) == w {
            l * h
        } else {
            w * h
        }
    }

    let x: i32 = l * w + l * h + w * h;
    let y: i32 = min_area(l, w, h);
    2 * x + y
}

fn length(l: i32, w: i32, h: i32) -> i32 {
    ribbon_length(l, w, h) + volume(l, w, h)
}

fn volume(l: i32, w: i32, h: i32) -> i32 {
    l * w * h
}

fn ribbon_length(l: i32, w: i32, h: i32) -> i32 {
    if max(l, max(w, h)) == h {
        2 * l + 2 * w
    } else if max(l, max(w, h)) == w {
        2 * l + 2 * h
    } else {
        2 * w + 2 * h
    }
}
