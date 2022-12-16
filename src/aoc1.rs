use std::cmp::Ordering;

use heapless::{binary_heap::Max, BinaryHeap};

pub fn part_1(input: &str) -> usize {
    let mut max = 0;
    let mut acc = 0;

    for line in input.lines() {
        if let Some(x) = line.trim().parse::<usize>().ok() {
            acc += x;
        } else {
            max = max.max(acc);
            acc = 0;
        }
    }
    max.max(acc)
}

pub fn part_2(input: &str) -> usize {
    let mut vals = Vec::new();
    let mut acc = 0;

    for line in input.lines() {
        if let Some(x) = line.trim().parse::<usize>().ok() {
            acc += x;
        } else {
            vals.push(acc);
            acc = 0;
        }
    }
    vals.push(acc);

    vals.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
    vals[0] + vals[1] + vals[2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(r#"1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000"# => 24000)]
    fn part1(input: &str) -> usize {
        part_1(input)
    }

    #[test_case(r#"1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000"# => 45000)]
    fn part2(input: &str) -> usize {
        part_2(input)
    }
}
