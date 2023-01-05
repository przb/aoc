pub mod day01 {
    use std::fs;

    pub fn getElf() {
        let input = fs::read_to_string("inputs/input01.txt").unwrap();
        let mut max = 0;
        for line in input.split("\n\n") {
            let mut line_total = 0;
            for line in line.split_ascii_whitespace() {
                let calories: i32 = line.parse().unwrap();
                line_total += calories;
            }
            if line_total > max {
                max = line_total;
            }
        }
        println!("Max = {max}");
    }
}