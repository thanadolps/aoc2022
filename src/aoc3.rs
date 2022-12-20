use itertools::Itertools;

/// Convert item (in ASCII) to priority.
///
/// - Lowercase item types a through z have priorities 1 through 26.
/// - Uppercase item types A through Z have priorities 27 through 52.
fn to_priority(item: u8) -> u8 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => panic!("Invalid item: {} (must be alphabet)", item),
    }
}

/// Calculate the sum of the priorities of the item types that appear in both compartments
/// of each rucksack in the input string.
///
/// The input string is made up of multiple lines, where each line represents a rucksack.
/// Each rucksack is made up of lowercase or uppercase letters representing the type of items
/// in the rucksack. The first half of the letters are in the first compartment and the second
/// half are in the second compartment.
pub fn part_1(input: &str) -> usize {
    // Initialize vectors to track which item types have been seen in each compartment of each rucksack.
    let mut seen_item_1 = vec![false; 52];
    let mut seen_item_2 = vec![false; 52];

    // Map each rucksack line to the sum of the priorities of the item types that appear in both compartments.
    input
        .lines()
        .map(|line| {
            // Reset the item type tracking vectors for each rucksack.
            seen_item_1.fill(false);
            seen_item_2.fill(false);

            // Split the line into the two compartments.
            let (items_1, items_2) = line.split_at(line.len() / 2);

            // Mark the item types seen in the first compartment.
            items_1.bytes().map(to_priority).for_each(|item_piority| {
                let item_index = (item_piority - 1) as usize;
                seen_item_1[item_index] = true;
            });

            // Sum the priorities of the item types that appear in both compartments.
            items_2
                .bytes()
                .map(to_priority)
                .filter_map(|item_piority| {
                    let item_index = (item_piority - 1) as usize;
                    if seen_item_1[item_index] & !seen_item_2[item_index] {
                        seen_item_2[item_index] = true;
                        Some(item_piority as usize)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        // Sum the results for each rucksack to get the final result.
        .sum()
}

pub fn part_2(input: &str) -> usize {
    // Initialize a 2D array with `false` values to keep track of the item types seen in each rucksack.
    // it has 3 rows and 52 columns. The rows represent the rucksacks and the columns represent the item types.
    //
    // Note that the item_index variable is calculated as (item_priority - 1) as usize,
    // which means that the item type with priority 1 will be stored at index 0 in the item_seens array
    let mut item_seens = vec![[false; 52]; 3];

    // Iterate over the input string by chunks of three lines (groups of three rucksacks).
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            // Reset the `item_seens` array for each group of rucksacks.
            item_seens.fill([false; 52]);

            // Iterate over each rucksack in the group.
            for (i, line) in group.enumerate() {
                // Iterate over each item in the rucksack.
                for item_priority in line.bytes().map(to_priority) {
                    // Calculate the index of the item type in the `item_seens` array.
                    let item_index = (item_priority - 1) as usize;
                    // Mark the item type as seen in the current rucksack.
                    item_seens[i][item_index] = true;

                    // If the item type has been seen in all three rucksacks, it's a badge item,
                    // return the item priority.
                    if item_seens[0][item_index]
                        && item_seens[1][item_index]
                        && item_seens[2][item_index]
                    {
                        return item_priority as usize;
                    }
                }
            }
            // If no badge item has been found, panic.
            panic!("Cannot determine badge item")
        })
        // Sum up all the badge item priorities.
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
    "#.trim() => 157; "part1")]
    fn part1(input: &str) -> usize {
        part_1(input)
    }

    #[test_case(r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#.trim() => 70; "part2")]
    fn part2(input: &str) -> usize {
        part_2(input)
    }
}
