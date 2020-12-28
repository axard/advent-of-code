fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../../res/aoc2015/5/input.txt"));
    let mut counter: usize = 0;

    for line in input.lines() {
        counter += if Stroka(line.to_string()).is_nice() {
            1
        } else {
            0
        };
    }

    println!("part 1: {}", counter);
}

struct Stroka(String);

impl Stroka {
    fn has_enough_vowels(&self) -> bool {
        let vowels = "aeiou";
        let mut counter = 3;
        for c in self.0.chars() {
            if vowels.contains(c) {
                counter -= 1;
                if counter == 0 {
                    return true;
                }
            }
        }

        return false;
    }

    fn has_double_letter(&self) -> bool {
        let mut prev: char = '\0';
        for c in self.0.chars() {
            if c == prev {
                return true;
            }

            prev = c;
        }

        return false;
    }

    fn has_deprecated(&self) -> bool {
        let deprecated = ["ab", "cd", "pq", "xy"];
        for d in deprecated.iter() {
            if self.0.contains(d) {
                return true;
            }
        }

        return false;
    }

    fn is_nice(&self) -> bool {
        self.has_enough_vowels() && self.has_double_letter() && !self.has_deprecated()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_enough_vowels() {
        assert_eq!(
            Stroka("ugknbfddgicrmopn".to_string()).has_enough_vowels(),
            true
        );
        assert_eq!(Stroka("aaa".to_string()).has_enough_vowels(), true);
        assert_eq!(
            Stroka("dvszwmarrgswjxmb".to_string()).has_enough_vowels(),
            false
        )
    }

    #[test]
    fn test_has_double_letter() {
        assert_eq!(
            Stroka("dvszwmarrgswjxmb".to_string()).has_double_letter(),
            true
        );
        assert_eq!(Stroka("aaa".to_string()).has_double_letter(), true);
        assert_eq!(
            Stroka("ugknbfddgicrmopn".to_string()).has_double_letter(),
            true
        );
        assert_eq!(Stroka("1234567890".to_string()).has_double_letter(), false);
        assert_eq!(
            Stroka("jchzalrnumimnmhp".to_string()).has_double_letter(),
            false
        );
    }

    #[test]
    fn test_has_deprecated() {
        assert_eq!(
            Stroka("haegwjzuvuyypxyu".to_string()).has_deprecated(),
            true
        );
        assert_eq!(
            Stroka("dvszwmarrgswjxmb".to_string()).has_deprecated(),
            false
        );
    }

    #[test]
    fn test_is_nice() {
        assert_eq!(Stroka("ugknbfddgicrmopn".to_string()).is_nice(), true);
        assert_eq!(Stroka("aaa".to_string()).is_nice(), true);
        assert_eq!(Stroka("jchzalrnumimnmhp".to_string()).is_nice(), false);
        assert_eq!(Stroka("haegwjzuvuyypxyu".to_string()).is_nice(), false);
        assert_eq!(Stroka("dvszwmarrgswjxmb".to_string()).is_nice(), false);
    }
}
