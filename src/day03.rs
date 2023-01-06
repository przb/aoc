pub mod day03 {
    use std::fs;

    macro_rules! priority {
        ($a:expr) => {
            if $a.is_uppercase() {
                $ a as i32 - 'A' as i32 + 26 + 1
            } else {
                $ a as i32 - 'a' as i32 + 1
            };
        }
    }



    pub fn rucksack() {
        let input = fs::read_to_string("inputs/input03.txt").unwrap();

        let mut total_priority = 0;
        'line_loop:
        for line in input.split("\n") {
            let first_half = &line[..line.len() / 2];
            let second_half = &line[line.len() / 2..];


            for letter in first_half.chars() {
                if second_half.contains(letter) {
                    let letter_priority = priority!(letter);
                    total_priority += letter_priority;
                    continue 'line_loop;
                }
            }

            for letter in second_half.chars() {
                if first_half.contains(letter) {
                    let letter_priority = priority!(letter);
                    total_priority += letter_priority;
                    continue 'line_loop;
                }
            }
        }
        println!("Day 3 part 1 solution: total priority = {total_priority}");
    }
}