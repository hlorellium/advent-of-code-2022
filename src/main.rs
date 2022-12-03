use advent_code::{count_total, priorities_sum, priorities_sum_3};

fn main() {
    // let max_calories = find_most_calories("task1_input.txt").unwrap();
    //
    // println!("{}", max_calories);
    //
    // println!("{}", find_top_n_most_calories("task1_input.txt", 3).unwrap());

    let total = priorities_sum_3("task3_input.txt").unwrap();
    // let total = count_total("task2_test.txt").unwrap();

    println!("Third day: task 1 {}", total);
}
