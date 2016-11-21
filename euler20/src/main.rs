/*n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!*/

extern crate num_bigint;
extern crate num_traits;
extern crate time;

use num_bigint::{BigInt, ToBigInt};
use num_traits::{Zero, One};
use time::precise_time_s;

fn factorial(num: &BigInt, counter: Option<BigInt>) -> BigInt {
    if num.is_zero() {
        counter.unwrap()
    }
    else {
        let running_total = num * counter.unwrap_or(BigInt::one());
        let next = num - BigInt::one();
        factorial(&next, Some(running_total))
    }
}


fn value_counter(fact: &str) -> u32 {
    let mut counter: u32 = 0;
    for c in fact.chars() {
        let x = c.to_digit(10).unwrap();
        counter += x;
    }
    counter
}

fn main() {
    let start_time = precise_time_s();
    let factorial_number = 100;
    let big_factorial = factorial_number.to_bigint().unwrap();
    let factorial_result: BigInt = factorial(&big_factorial, None);
    let character_adding_result: u32 = value_counter(&factorial_result.to_string());
    let end_time = precise_time_s();


    println!("Factorial number: {}\nTime taken: {} seconds", character_adding_result, (end_time-start_time));
}
