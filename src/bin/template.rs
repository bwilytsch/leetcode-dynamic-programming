use std::time::Instant;

pub struct Solution {}

impl Solution {
    pub fn run(n: i32) -> i32 {
        n
    }
}

fn main() {
    let timer = Instant::now();
    let result = Solution::run(0);
    let duration = timer.elapsed();
    println!("Result: {} - {:?}", result, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(Solution::run(0), 0);
    }
}
