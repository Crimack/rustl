
use std::time::SystemTime;

extern crate time;

// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn brute_force(max_num: i32) -> i32 {
    // Brute force solution
    let mut found_number = false;
    let mut current: i32 = 0;

    while !found_number {
        current += 1;
        // println!("Current: {}", current);
        for i in 1..max_num+1 {
            if current % i != 0 {
                break;
            }
            else
            {
                if i == max_num { found_number = true }
            }
        }
    }
    current
}


fn main(){
    let max = 20;
    let brute_force_start_time = time::now();
    let result = brute_force(max);
    let brute_force_end_time = time::now();
    println!("Result: {}", result);
    println!("Brute force start: {}:{}", brute_force_start_time.tm_min, brute_force_start_time.tm_sec);
    println!("Brute force end: {}:{}", brute_force_end_time.tm_min, brute_force_end_time.tm_sec);
}
