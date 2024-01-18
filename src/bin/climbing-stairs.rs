use std::collections::HashMap;
use std::time::Instant;

type Memo = HashMap<i32, i32>;

// This is basically a fibonacci sequence
fn v1(n: i32) -> Result<i32, String> {
    let mut memo: Memo = HashMap::new();

    fn step(steps: i32, memo: &mut Memo) -> i32 {
        match steps {
            1 => 1,
            2 => 2,
            _ => match memo.get(&steps) {
                Some(&v) => v,
                None => {
                    let v = step(steps - 1, memo) + step(steps - 2, memo);
                    memo.insert(steps, v);
                    v
                }
            },
        }
    }

    Ok(step(n, &mut memo))
}

fn v2(n: i32) -> Result<i32, String> {
    let mut cache = [0i32; 46];
    cache[0] = 1;
    cache[1] = 1;

    fn climb(n: i32, cache: &mut [i32; 46]) -> i32 {
        let mut val = cache[n as usize];

        if val != 0 {
            return val;
        }

        val = climb(n - 1, cache) + climb(n - 2, cache);
        cache[n as usize] = val;
        val
    }

    Ok(climb(n, &mut cache))
}

fn main() {
    let start = Instant::now();
    if let Ok(output) = v1(45) {
        let duration = start.elapsed();
        println!("Output v1: {} in {:?}", output, duration);
    }

    if let Ok(output) = v2(45) {
        let duration = start.elapsed();
        println!("Output v2: {} in {:?}", output, duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_2() {
        assert_eq!(v1(2).unwrap(), 2);
    }

    #[test]
    fn base_3() {
        assert_eq!(v1(3).unwrap(), 3);
    }
}
