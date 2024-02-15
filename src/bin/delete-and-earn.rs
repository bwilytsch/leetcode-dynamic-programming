use std::{cmp::max, collections::HashMap, time::Instant};

pub struct Solution {}

// Problem: https://leetcode.com/problems/delete-and-earn

// 1 <= nums.length <= 2 * 10^4
// 1 <= nums[i] <= 2 * 10^4

impl Solution {
    pub fn delete_and_earn_slow(mut nums: Vec<i32>) -> i32 {
        let mut memo: HashMap<String, i32> = HashMap::new();

        fn walk(i: usize, values: &Vec<i32>, memo: &mut HashMap<String, i32>) -> i32 {
            if i >= values.len() {
                return 0;
            }

            let current_value = values[i];
            let hash = format!("{}-{:?}", current_value, values);

            if let Some(value) = memo.get(&hash) {
                return *value;
            }

            let pick_current = current_value
                + walk(
                    0,
                    &values
                        .iter()
                        .enumerate()
                        .filter_map(|(j, &n)| {
                            if n == current_value + 1 || n == current_value - 1 || i == j {
                                return None;
                            }

                            Some(n)
                        })
                        .collect::<Vec<_>>(),
                    memo,
                );
            let skip_current = walk(i + 1, values, memo);

            let result = pick_current.max(skip_current);

            memo.insert(hash, result);

            result
        }

        nums.sort();

        walk(0, &nums, &mut memo)
    }

    // [3, 4, 2]
    fn delete_and_earn_fast(mut nums: Vec<i32>) -> i32 {
        // Operating on a sorted array
        // Sort it to remove the logic for removing N-1
        nums.sort();

        // Points
        let mut pp = -1;
        let mut p = nums[0];

        // Results
        let mut s1 = 0;
        let mut s2 = 0;

        // Buffer
        let mut s = 0;

        // E.g. [2,3,4]
        for i in 0..nums.len() {
            // Check if we hit a number that is bigger than previously selected point p to meet the (N + 1) case
            if nums[i] != p {
                if p > pp + 1 && nums[i] > p + 1 {
                    // Number is high enough to be considered for the (N + 1) case and both results
                    s2 += s;
                    s1 += s;
                } else {
                    // Number is not high enough to be considered for the (N + 1) case, so it's
                    // only added to the first result
                    s1 += s;

                    let t = if p > pp + 1 {
                        // This condition usually only happens on the *second* step, because pp = -1
                        max(s1, s2 + s)
                    } else {
                        max(s1, s2)
                    };

                    s1 = s2;
                    s2 = t;
                }

                s = 0;
                // Reference to previous value of p
                pp = p;
                // Reference to last points
                p = nums[i];
            }

            // Accumulate points of the same number
            s += nums[i];
        }

        if p > pp + 1 {
            // Number is high enough to be considered for the (N + 1) case and both results
            max(s1 + s, s2 + s)
        } else {
            max(s1 + s, s2)
        }
    }
}

fn main() {
    Solution::delete_and_earn_fast([3, 4, 2].to_vec());

    let timer = Instant::now();
    let result = Solution::delete_and_earn_slow(
        [
            6, 9, 6, 7, 8, 10, 5, 2, 7, 2, 3, 1, 5, 5, 6, 10, 8, 6, 4, 10, 1, 6, 6, 7, 4, 3, 7, 9,
            10, 2, 5, 9, 9, 8, 2, 10, 4, 2, 2, 5, 10, 6, 10, 1, 10, 4, 5, 1, 8, 6,
        ]
        .to_vec(),
    );
    let duration = timer.elapsed();
    println!("Result HashMap: {} - {:?}", result, duration);

    let timer = Instant::now();
    let result = Solution::delete_and_earn_fast(
        [
            6, 9, 6, 7, 8, 10, 5, 2, 7, 2, 3, 1, 5, 5, 6, 10, 8, 6, 4, 10, 1, 6, 6, 7, 4, 3, 7, 9,
            10, 2, 5, 9, 9, 8, 2, 10, 4, 2, 2, 5, 10, 6, 10, 1, 10, 4, 5, 1, 8, 6,
        ]
        .to_vec(),
    );
    let duration = timer.elapsed();
    println!("Result Array: {} - {:?}", result, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exmpale_one() {
        assert_eq!(Solution::delete_and_earn_slow(vec![3, 4, 2]), 6);
    }

    #[test]
    fn exmpale_two() {
        assert_eq!(Solution::delete_and_earn_slow(vec![2, 2, 3, 3, 3, 4]), 9);
    }
}
