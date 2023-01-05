pub mod day02 {
    use std::collections::HashMap;
    use std::fs;

    const MY_ROCK: char = 'X';      // lose for pt2
    const MY_PAPER: char = 'Y';     // win for pt2
    const MY_SCISSOR: char = 'Z';   // draw for pt2

    const ROCK_POINTS: i32 = 1;
    const PAPER_POINTS: i32 = 2;
    const SCISSORS_POINTS: i32 = 3;

    const WIN_POINTS: i32 = 6;
    const TIE_POINTS: i32 = 3;
    const LOSS_POINTS: i32 = 0;

    const OPPONENT_ROCK: char = 'A';
    const OPPONENT_PAPER: char = 'B';
    const OPPONENT_SCISSORS: char = 'C';


    pub fn rock_paper_scissors() {
        let mut scores = HashMap::new();
        scores.insert(String::from("A X"), TIE_POINTS+ROCK_POINTS);
        scores.insert(String::from("B X"), LOSS_POINTS+ROCK_POINTS);
        scores.insert(String::from("C X"), WIN_POINTS+ROCK_POINTS);

        scores.insert(String::from("A Y"), WIN_POINTS+PAPER_POINTS);
        scores.insert(String::from("B Y"), TIE_POINTS+PAPER_POINTS);
        scores.insert(String::from("C Y"), LOSS_POINTS+PAPER_POINTS);

        scores.insert(String::from("A Z"), LOSS_POINTS+SCISSORS_POINTS);
        scores.insert(String::from("B Z"), WIN_POINTS+SCISSORS_POINTS);
        scores.insert(String::from("C Z"), TIE_POINTS+SCISSORS_POINTS);

        let input = fs::read_to_string("inputs/input02.txt").unwrap();

        let mut total = 0;
        for line in input.split("\n") {
            total += scores.get(line).unwrap_or(&0);
        }

        println!("Day 2 part 1 solution: total points = {total}");
    }
}