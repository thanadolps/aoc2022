use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| {
                    line.parse::<usize>()
                        .expect("Each line must be an interger")
                })
                .sum()
        })
        .max()
        .expect("Input isn't empty")
}

pub fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| {
                    line.parse::<usize>()
                        .expect("Each line must be an interger")
                })
                .sum::<usize>()
        })
        .map(std::cmp::Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#.trim() => 24000; "part1")]
    fn part1(input: &str) -> usize {
        part_1(input)
    }

    #[test_case(r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#.trim() => 45000; "part2")]
    fn part2(input: &str) -> usize {
        part_2(input)
    }
}
