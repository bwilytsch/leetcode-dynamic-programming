use std::collections::HashMap;
use std::time::Instant;

type Memo = HashMap<i32, i32>;

pub struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut memo: Memo = HashMap::new();
        memo.insert(0, 0);
        memo.insert(1, 1);

        fn calc(n: i32, memo: &mut Memo) -> i32 {
            match memo.get(&n) {
                Some(&v) => v,
                None => {
                    let v = calc(n - 1, memo) + calc(n - 2, memo);
                    memo.insert(n, v);
                    v
                }
            }
        }

        calc(n, &mut memo)
    }

    pub fn fib_faster(n: i32) -> i32 {
        let mut fib_vec = Vec::new();
        fib_vec.push(0);
        fib_vec.push(1);

        match n {
            0 => return 0,
            1 => return 1,
            _ => {
                for i in 0..n {
                    fib_vec.push(fib_vec[i as usize] + fib_vec[i as usize + 1]);
                }
            }
        }

        return fib_vec[n as usize];
    }

    pub fn fib_small(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }

        let (mut a, mut b) = (0, 1);
        #[allow(unused_assignments)]
        let mut tmp = 0;

        for _ in 1..n {
            tmp = a + b;
            a = b;
            b = tmp;
        }

        b
    }
}

fn main() {
    let sample = 120000;

    println!("Fibonacci of {}", 120000);

    let start = Instant::now();
    let result = Solution::fib(sample);
    let duration = start.elapsed();
    println!("Result fib: {} - {:?}", result, duration);

    let start = Instant::now();
    let result = Solution::fib_faster(sample);
    let duration = start.elapsed();
    println!("Result fib_faster: {} - {:?}", result, duration);

    let start = Instant::now();
    let result = Solution::fib_small(sample);
    let duration = start.elapsed();
    println!("Result fib_small: {} - {:?}", result, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::fib(2), 1);
    }
}
