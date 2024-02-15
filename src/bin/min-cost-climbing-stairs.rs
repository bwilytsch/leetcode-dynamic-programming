use std::{cmp::min, time::Instant};

pub struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let count = cost.len();

        let mut dp: Vec<i32> = vec![0; count + 1];

        dp[count] = 0;
        dp[count - 1] = cost[count - 1];

        for i in (0..count - 1).rev() {
            dp[i] = cost[i] + min(dp[i + 1], dp[i + 2]);
        }

        min(dp[0], dp[1])
    }

    pub fn min_cost_climbing_stairs_faster(cost: Vec<i32>) -> i32 {
        let stair_count = cost.len();
        if stair_count < 2 {
            return 0;
        }

        let mut cost_history = vec![0; stair_count];
        cost_history[0] = cost[0];
        cost_history[1] = cost[1];

        for i in 2..stair_count {
            cost_history[i] = cost[i] + cost_history[i - 1].min(cost_history[i - 2]);
        }

        cost_history[stair_count - 1].min(cost_history[stair_count - 2])
    }
}

fn main() {
    let timer = Instant::now();
    let result = Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]);
    let duration = timer.elapsed();
    println!("Result: {} - {:?}", result, duration);

    let timer = Instant::now();
    let result =
        Solution::min_cost_climbing_stairs_faster(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]);
    let duration = timer.elapsed();
    println!("Result Faster: {} - {:?}", result, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn advanced() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
