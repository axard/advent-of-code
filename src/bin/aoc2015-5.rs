fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../../res/aoc2015/5/input.txt"));
    let mut counter_by_old_rules: usize = 0;
    let mut counter_by_new_rules: usize = 0;

    for line in input.lines() {
        counter_by_old_rules += if Stroka(line.clone().to_string()).is_nice(Rule::Old) {
            1
        } else {
            0
        };

        counter_by_new_rules += if Stroka(line.clone().to_string()).is_nice(Rule::New) {
            1
        } else {
            0
        };
    }

    println!("part 1: {}", counter_by_old_rules);
    println!("part 2: {}", counter_by_new_rules);
}

#[derive(Debug)]
struct Stroka(String);

#[derive(Eq, PartialEq)]
enum Rule {
    Old,
    New,
}

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

    fn has_pair_appears_twice(&self) -> bool {
        if self.0.len() < 4 {
            return false;
        }

        let mut chars = self.0.chars();

        while let Some(fst) = chars.next() {
            let mut chars = chars.clone();
            if let Some(snd) = chars.next() {
                while let Some(fst_) = chars.next() {
                    if fst_ == fst {
                        if let Some(snd_) = chars.clone().next() {
                            if snd_ == snd {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    fn has_letter_repeat_after_other(&self) -> bool {
        let mut buffer: [char; 2] = ['\0', '\0'];
        let mut chars = self.0.chars();

        if let Some(letter) = chars.next() {
            buffer[0] = letter
        } else {
            return false;
        }

        if let Some(letter) = chars.next() {
            buffer[1] = letter
        } else {
            return false;
        }

        while let Some(letter) = chars.next() {
            if letter == buffer[0] {
                return true;
            }

            buffer[0] = buffer[1];
            buffer[1] = letter;
        }

        false
    }

    fn is_nice(&self, rule: Rule) -> bool {
        if rule == Rule::Old {
            self.has_enough_vowels() && self.has_double_letter() && !self.has_deprecated()
        } else {
            self.has_letter_repeat_after_other() && self.has_pair_appears_twice()
        }
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
    fn test_is_nice_by_old_rule() {
        assert_eq!(
            Stroka("ugknbfddgicrmopn".to_string()).is_nice(Rule::Old),
            true
        );
        assert_eq!(Stroka("aaa".to_string()).is_nice(Rule::Old), true);
        assert_eq!(
            Stroka("jchzalrnumimnmhp".to_string()).is_nice(Rule::Old),
            false
        );
        assert_eq!(
            Stroka("haegwjzuvuyypxyu".to_string()).is_nice(Rule::Old),
            false
        );
        assert_eq!(
            Stroka("dvszwmarrgswjxmb".to_string()).is_nice(Rule::Old),
            false
        );
    }

    #[test]
    fn test_has_letter_repeat_after_other() {
        assert_eq!(
            Stroka("aaa".to_string()).has_letter_repeat_after_other(),
            true
        );
        assert_eq!(
            Stroka("xyx".to_string()).has_letter_repeat_after_other(),
            true
        );
        assert_eq!(
            Stroka("abcdefeghi".to_string()).has_letter_repeat_after_other(),
            true
        );
        assert_eq!(
            Stroka("uurcxstgmygtbstg".to_string()).has_letter_repeat_after_other(),
            false
        );
    }

    #[test]
    fn test_is_nice_by_new_rule() {
        assert_eq!(
            Stroka("qjhvhtzxzqqjkmpb".to_string()).is_nice(Rule::New),
            true
        );
        assert_eq!(Stroka("xxyxx".to_string()).is_nice(Rule::New), true);
        assert_eq!(
            Stroka("uurcxstgmygtbstg".to_string()).is_nice(Rule::New),
            false
        );
        assert_eq!(
            Stroka("ieodomkazucvgmuy".to_string()).is_nice(Rule::New),
            false
        );
    }

    #[test]
    fn test_has_pair_appears_twice() {
        assert_eq!(
            // повторяется 'qj' ==> true
            Stroka("qjhvhtzxzqqjkmpb".to_string()).has_pair_appears_twice(),
            true
        );
        assert_eq!(
            // повторяется 'xx' ==> true
            Stroka("xxyxx".to_string()).has_pair_appears_twice(),
            true
        );
        assert_eq!(
            // повторяется с наложением 'bb' ==> false
            Stroka("abbbz".to_string()).has_pair_appears_twice(),
            false
        );
    }
}
