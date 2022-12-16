extern crate core;

mod aoc1;
// mod aoc2;
// mod aoc3;
// mod aoc4;
// mod aoc5;
// mod aoc6;

fn main() {
    generate_benchmark()
}

fn generate_benchmark() {
    let mut table = comfy_table::Table::new();
    table
        .set_header(["Name", "Runtime", "Output"])
        .load_preset(comfy_table::presets::ASCII_MARKDOWN);

    let mut total_ns = 0.;

    for (name, input_path, f) in [
        (
            "Day 1: Calorie Counting, Part 1",
            "input/aoc1.txt",
            aoc1::part_1 as fn(&str) -> usize,
        ),
        (
            "Day 1: Calorie Counting, Part 2",
            "input/aoc1.txt",
            aoc1::part_2,
        ),
        // ("Day 2: Dive!, Part 1", "input/aoc2.txt", aoc2::part_1),
        // ("Day 2: Dive!, Part 2", "input/aoc2.txt", aoc2::part_2),
        // (
        //     "Day 3: Binary Diagnostic, Part 1",
        //     "input/aoc3.txt",
        //     aoc3::part_1,
        // ),
        // (
        //     "Day 3: Binary Diagnostic, Part 2",
        //     "input/aoc3.txt",
        //     aoc3::part_2,
        // ),
        // ("Day 4: Giant Squid, Part 1", "input/aoc4.txt", aoc4::part_1),
        // ("Day 4: Giant Squid, Part 2", "input/aoc4.txt", aoc4::part_2),
        // (
        //     "Day 5: Hydrothermal Venture, Part 1",
        //     "input/aoc5.txt",
        //     aoc5::part_1,
        // ),
        // (
        //     "Day 5: Hydrothermal Venture, Part 2",
        //     "input/aoc5.txt",
        //     aoc5::part_2,
        // ),
        // ("Day 6: Lanternfish, Part 1", "input/aoc6.txt", aoc6::part_1),
        // ("Day 6: Lanternfish, Part 2", "input/aoc6.txt", aoc6::part_2),
    ] {
        let input = std::fs::read_to_string(input_path).unwrap();
        let bench_stats = easybench::bench(|| f(&input));
        total_ns += bench_stats.ns_per_iter;

        table.add_row([
            name.to_string(),
            bench_stats.to_string(),
            f(&input).to_string(),
        ]);
    }

    let total_time = std::time::Duration::from_nanos(total_ns as u64);
    table.add_row([
        "Total".to_string(),
        format!("{:?}", total_time),
        String::new(),
    ]);

    println!("{}", table);
}
