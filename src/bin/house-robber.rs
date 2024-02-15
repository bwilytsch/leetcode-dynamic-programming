use std::time::Instant;

pub struct Solution {}

// Input: [2, 1, 1, 2]
// O: 2 + return of i = 2 vs return of i = 1; -> [4, 3, 2, 2]
// 1: 1 + return of i = 3 vs return of i = 2; -> [-1, 3, 2, 2]
// 2: 1 + return of i = 4(0) vs return of i = 3; -> [-1, -1, 2, 2]
// 3: 2 + return of i = 5(0) vs return of i = 4(0); -> [-1, -1, -1, 2]

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut memo = vec![-1; n];

        fn walk(i: usize, values: &Vec<i32>, memo: &mut Vec<i32>) -> i32 {
            if i >= values.len() {
                return 0;
            }

            if memo[i] != -1 {
                return memo[i];
            }

            let rob_current = values[i] as i32 + walk(i + 2, values, memo);
            let skip_current = walk(i + 1, values, memo);

            let result = rob_current.max(skip_current);
            memo[i] = result;

            result
        }

        walk(0, &nums, &mut memo)
    }

    pub fn rob_short(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |(pp, p), curr| (p, p.max(curr + pp)))
            .1
    }

    pub fn rob_faster(nums: Vec<i32>) -> i32 {
        let mut robbed_idx_plus1 = 0;
        let mut robbed_idx_plus2 = 0;

        for i in (0..nums.len() - 1).rev() {
            let sum_if_skipped = robbed_idx_plus1;
            let sum_if_robbed = nums[i] + robbed_idx_plus2;

            let max_robbed_at_idx = sum_if_skipped.max(sum_if_robbed);

            robbed_idx_plus2 = robbed_idx_plus1;
            robbed_idx_plus1 = max_robbed_at_idx;
        }

        return robbed_idx_plus1;
    }
}

fn main() {
    let timer = Instant::now();
    let result = Solution::rob(vec![2, 7, 9, 3, 1]);
    let duration = timer.elapsed();
    println!("Result: {} - {:?}", result, duration);

    let timer = Instant::now();
    let result = Solution::rob_short(vec![2, 7, 9, 3, 1]);
    let duration = timer.elapsed();
    println!("Result: {} - {:?}", result, duration);

    let timer = Instant::now();
    let result = Solution::rob_faster(vec![2, 7, 9, 3, 1]);
    let duration = timer.elapsed();
    println!("Result: {} - {:?}", result, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::rob([1, 2, 3, 1].to_vec()), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::rob([2, 7, 9, 3, 1].to_vec()), 12);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::rob([1, 3, 1].to_vec()), 3);
    }

    #[test]
    fn example_four() {
        assert_eq!(Solution::rob_faster([2, 1, 1, 2].to_vec()), 4);
    }
}
