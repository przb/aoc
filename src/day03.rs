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
        let mut badge_priority = 0;
        let mut overlap = String::new();
        'line_loop:
        for (i, line) in input.lines().enumerate() {
            if i % 3 == 0 {
                if i > 0 { badge_priority += priority!(overlap.chars().nth(0).unwrap()); }
                overlap = String::from(line);
            };
            overlap.retain(|c| line.contains(c));


            let first_half = &line[..line.len() / 2];
            let second_half = &line[line.len() / 2..];

            if i == 299 {
                badge_priority += priority!(overlap.chars().nth(0).unwrap());
            }

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
        println!("Day 3 part 2 solution: total priority = {badge_priority}");
    }
}