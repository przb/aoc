pub mod day02 {
    use std::collections::HashMap;
    use std::fs;

    // const MY_ROCK: char = 'X';      // lose for pt2
    // const MY_PAPER: char = 'Y';     // win for pt2
    // const MY_SCISSOR: char = 'Z';   // draw for pt2

    const ROCK_POINTS: i32 = 1;
    const PAPER_POINTS: i32 = 2;
    const SCISSORS_POINTS: i32 = 3;

    const WIN_POINTS: i32 = 6;
    const TIE_POINTS: i32 = 3;
    const LOSS_POINTS: i32 = 0;

    // const OPPONENT_ROCK: char = 'A';
    // const OPPONENT_PAPER: char = 'B';
    // const OPPONENT_SCISSORS: char = 'C';


    pub fn rock_paper_scissors() {
        let t1 = std::time::SystemTime::now();

        let mut scores_part1 = HashMap::new();
        scores_part1.insert(String::from("A X"), TIE_POINTS+ROCK_POINTS);
        scores_part1.insert(String::from("B X"), LOSS_POINTS+ROCK_POINTS);
        scores_part1.insert(String::from("C X"), WIN_POINTS+ROCK_POINTS);

        scores_part1.insert(String::from("A Y"), WIN_POINTS+PAPER_POINTS);
        scores_part1.insert(String::from("B Y"), TIE_POINTS+PAPER_POINTS);
        scores_part1.insert(String::from("C Y"), LOSS_POINTS+PAPER_POINTS);

        scores_part1.insert(String::from("A Z"), LOSS_POINTS+SCISSORS_POINTS);
        scores_part1.insert(String::from("B Z"), WIN_POINTS+SCISSORS_POINTS);
        scores_part1.insert(String::from("C Z"), TIE_POINTS+SCISSORS_POINTS);

        let mut scores_part2 = HashMap::new();
        scores_part2.insert(String::from("A X"), LOSS_POINTS+SCISSORS_POINTS);
        scores_part2.insert(String::from("B X"), LOSS_POINTS+ROCK_POINTS);
        scores_part2.insert(String::from("C X"), LOSS_POINTS+PAPER_POINTS);

        scores_part2.insert(String::from("A Y"), TIE_POINTS+ROCK_POINTS);
        scores_part2.insert(String::from("B Y"), TIE_POINTS+PAPER_POINTS);
        scores_part2.insert(String::from("C Y"), TIE_POINTS+SCISSORS_POINTS);

        scores_part2.insert(String::from("A Z"), WIN_POINTS+PAPER_POINTS);
        scores_part2.insert(String::from("B Z"), WIN_POINTS+SCISSORS_POINTS);
        scores_part2.insert(String::from("C Z"), WIN_POINTS+ROCK_POINTS);

        let input = fs::read_to_string("inputs/input02.txt").unwrap();

        let mut total_part1 = 0;
        let mut total_part2 = 0;
        for line in input.split("\n") {
            total_part1 += scores_part1.get(line).unwrap_or(&0);
            total_part2 += scores_part2.get(line).unwrap_or(&0);
        }
        let t2 = std::time::SystemTime::now();
        let ms_compute = t2.duration_since(t1).unwrap().as_micros();

        println!("Day 2 part 1 solution: total points = {total_part1}");
        println!("Day 2 part 2 solution: total points = {total_part2}");
        println!("Took {ms_compute}μs");

    }
}