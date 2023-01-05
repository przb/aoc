pub mod day01 {
    use std::cmp::Reverse;
    use std::fs;
    use std::collections::{binary_heap, BinaryHeap};

    pub fn get_max_elf() {

        let t1 = std::time::SystemTime::now();

        let num_max = 3;
        let mut heap = BinaryHeap::new();
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
            if heap.len() < num_max {
                heap.push(Reverse(line_total)); // Wrapping line_total in reverse makes this a min heap
            } else if Reverse(line_total) < *heap.peek().unwrap() {
                heap.pop();
                heap.push(Reverse(line_total));
            }
        }

        let mut part_two_sum = 0;
        for value in heap.iter(){
            part_two_sum += value.0;
        }

        let t2 = std::time::SystemTime::now();

        let ms_compute = t2.duration_since(t1).unwrap().as_micros();
        println!("Day 1 part 1 solution: Max = {max}");
        println!("Day 1 part 2 solution: Max = {part_two_sum}");
        println!("Took {ms_compute}μs");
    }
}
