// This function parses an assignment pair string and returns a tuple of RangeInclusive<u32> objects.
fn parse_assignment_pair(
    pair: &str,
) -> (std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>) {
    // Split the assignment pair into the two assignments
    let (assignment_1, assignment_2) = pair
        .split_once(',')
        .expect("Each assignment pair should have two section assignments");

    // Split the first assignment into its start and end sections
    let sections_1 = assignment_1.split_once('-').unwrap();
    // Parse the start and end sections as u32 values and create a range inclusive of both
    let sections_1: std::ops::RangeInclusive<u32> =
        sections_1.0.parse().unwrap()..=sections_1.1.parse().unwrap();

    // Split the second assignment into its start and end sections
    let sections_2 = assignment_2.split_once('-').unwrap();
    // Parse the start and end sections as u32 values and create a range inclusive of both
    let sections_2: std::ops::RangeInclusive<u32> =
        sections_2.0.parse().unwrap()..=sections_2.1.parse().unwrap();

    // Return the parsed ranges as a tuple
    (sections_1, sections_2)
}

// This function calculates the number of assignment pairs in which one range fully contains the other
// in the given input string.
pub fn part_1(input: &str) -> usize {
    // Parse each line of the input string as an assignment pair
    input
        .lines()
        // Map each assignment pair to a value indicating whether one range fully contains the other
        .map(|line| {
            // Parse the assignment pair into two RangeInclusive<u32> objects
            let (sections_1, sections_2) = parse_assignment_pair(line);

            // Check if one range fully contains the other
            let is_fully_contain = (sections_1.start() <= sections_2.start()
                && sections_1.end() >= sections_2.end())
                || (sections_2.start() <= sections_1.start()
                    && sections_2.end() >= sections_1.end());

            // Return a boolean as an usize
            is_fully_contain as usize
        })
        // Sum the values to get the final result
        .sum()
}

// This function calculates the number of assignment pairs in which the ranges have any overlap
// in the given input string.
pub fn part_2(input: &str) -> usize {
    // Parse each line of the input string as an assignment pair
    input
        .lines()
        // Map each assignment pair to a value indicating whether the ranges have any overlap
        .map(|line| {
            // Parse the assignment pair into two RangeInclusive<u32> objects
            let (sections_1, sections_2) = parse_assignment_pair(line);

            // Check if the ranges intersect
            let has_overlap = (sections_1.start() <= sections_2.end()
                && sections_1.end() >= sections_2.start())
                || (sections_2.start() <= sections_1.end()
                    && sections_2.end() >= sections_1.start());

            // Return a boolean as an usize
            has_overlap as usize
        })
        // Sum the values to get the final result
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#.trim() => 2; "part1")]
    #[test_case(r#"
2-40,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#.trim() => 3; "part1_multidigit")]
    fn part1(input: &str) -> usize {
        part_1(input)
    }

    #[test_case(r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#.trim() => 4; "part2")]
    fn part2(input: &str) -> usize {
        part_2(input)
    }
}
