use std::time::Instant;

pub struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }

        let (mut a, mut b, mut c) = (0, 1, 1);
        #[allow(unused_assignments)]
        let mut tmp = 0;

        for _ in 2..n {
            tmp = a + b + c;
            a = b;
            b = c;
            c = tmp;
        }

        c
    }
}

// 0 <= n <= 37 and is guaranteed to fit into a 32-bit intiger
fn main() {
    let samples = 37;

    let timer = Instant::now();
    let result = Solution::tribonacci(samples);
    let duration = timer.elapsed();
    println!("Result trib: {} -  {:?}", result, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neq3() {
        assert_eq!(Solution::tribonacci(3), 2)
    }

    #[test]
    fn neq4() {
        assert_eq!(Solution::tribonacci(4), 4)
    }

    #[test]
    fn neq25() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
