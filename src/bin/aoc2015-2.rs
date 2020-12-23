use std::cmp::max;

fn main() {
    let input = include_bytes!("../../res/aoc2015/2/input.txt");
    let input = String::from_utf8_lossy(input);

    let mut full_area: i32 = 0;
    for line in input.lines() {
        let values: Vec<&str> = line.split('x').collect();
        if values.len() == 3 {
            let l = values[0].parse::<i32>().unwrap();
            let w = values[1].parse::<i32>().unwrap();
            let h = values[2].parse::<i32>().unwrap();

            full_area += area(l, w, h)
        }
    }

    println!("{}", full_area)
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
    let x: i32 = l * w + l * h + w * h;
    let y: i32 = min(l, w, h);
    2 * x + y
}

fn min(l: i32, w: i32, h: i32) -> i32 {
    if max(l, max(w, h)) == h {
        l * w
    } else if max(l, max(w, h)) == w {
        l * h
    } else {
        w * h
    }
}
