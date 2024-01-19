use std::{collections::HashMap, time::Instant};

pub struct Solution {}

type Point = (i32, i32);
const BOARD: [&str; 6] = ["abcde", "fghij", "klmno", "pqrst", "uvwxy", "z"];

pub fn distance((x0, y0): &Point, (x1, y1): &Point) -> Point {
    (x0 - x1, y0 - y1)
}

impl Solution {
    pub fn run(target: String) -> String {
        let mut letters: HashMap<char, Point> = HashMap::new();

        for (i, v) in BOARD.iter().enumerate() {
            let chars = v.chars().collect::<Vec<_>>();
            for j in 0..v.len() {
                letters.insert(chars[j], (j as i32, i as i32));
            }
        }

        let positions = target
            .chars()
            .map(|c| match letters.get(&c) {
                Some(p) => p.clone(),
                None => (0, 0),
            })
            .collect::<Vec<Point>>();

        let mut current: Point = (0, 0);
        let mut output = "".to_owned();

        fn can_move((x, y): Point) -> bool {
            let row = BOARD.get(y as usize);

            if row.is_none() {
                return false;
            }

            row.unwrap().chars().count() > (x as usize)
        }

        fn walk(target: &Point, location: Point) -> String {
            let (dx, dy) = distance(&target, &location);

            if dy < 0 && can_move((location.0, location.1 - 1)) {
                return "U".to_owned() + &walk(target, (location.0, location.1 - 1));
            }

            if dy > 0 && can_move((location.0, location.1 + 1)) {
                return "D".to_owned() + &walk(target, (location.0, location.1 + 1));
            }

            if dx < 0 && can_move((location.0 - 1, location.1)) {
                return "L".to_owned() + &walk(target, (location.0 - 1, location.1));
            }

            if dx > 0 && can_move((location.0 + 1, location.1)) {
                return "R".to_owned() + &walk(target, (location.0 + 1, location.1));
            }

            return "!".to_owned();
        }

        for (_, p) in positions.iter().enumerate() {
            output.push_str(&walk(p, current));

            current = *p;
        }

        output
    }

    fn run_alt(target: String) -> String {
        // ascii a to z:
        //  97  98  99 100 101
        // 102 103 104 105 106
        // 107 108 109 110 111
        // 112 113 114 115 116
        // 117 118 119 120 121
        // 122

        let mut start: u8 = 97; // starting point
        let mut result: String = String::from("");

        for c in target.chars() {
            let dest: u8 = c as u8;

            while start != dest {
                let current_row: u8 = (start - 97) / 5;
                let target_row: u8 = (dest - 97) / 5;

                if current_row > target_row {
                    result.push('U');
                    start -= 5;
                } else if current_row < target_row {
                    if start + 5 > 122 {
                        result.push('L');
                        start -= 1;
                    } else {
                        result.push('D');
                        start += 5;
                    }
                } else {
                    if start > dest {
                        result.push('L');
                        start -= 1;
                    } else {
                        result.push('R');
                        start += 1;
                    }
                }
            }

            result.push('!');
        }

        result
    }
}

fn main() {
    let timer = Instant::now();
    let result = Solution::run("zdz".to_owned());
    let duration = timer.elapsed();
    println!("Result: {} - {:?}", result, duration);

    let timer = Instant::now();
    let result = Solution::run_alt("zdz".to_owned());
    let duration = timer.elapsed();
    println!("Result: {} - {:?}", result, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leet() {
        assert_eq!(
            Solution::run("leet".to_owned()),
            "DDR!UURRR!!DDD!".to_owned()
        );
    }

    #[test]
    fn code() {
        assert_eq!(
            Solution::run("code".to_owned()),
            "RR!DDRR!UUL!R!".to_owned()
        );
    }

    #[test]
    fn zdz() {
        assert_eq!(
            Solution::run("zdz".to_owned()),
            "DDDDD!UUUUURRR!DDDDLLLD!".to_owned()
        );
    }
}
