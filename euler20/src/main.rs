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

fn factorial(num : &BigInt, zero: &BigInt, one: &BigInt) -> BigInt {
    if num == zero {
        One::one()
    }
    else {
        let next = num - one;
        num * factorial(&next, zero, one)
    }
}


fn value_counter(fact: &str) -> i32 {
    let mut counter: i32 = 0;
    for c in fact.chars() {
        // Can't cast directly from char to int, so have to do some magic
        let num_c = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => 0,
        };
        counter += num_c;
    }
    counter
}

fn main() {
    let start_time = precise_time_s();
    let factorial_number = 100;
    let big_factorial = factorial_number.to_bigint().unwrap();
    let zero = Zero::zero();
    let one = One::one();
    let factorial_result: BigInt = factorial(&big_factorial, &zero, &one);
    let character_adding_result: i32 = value_counter(&factorial_result.to_string());
    let end_time = precise_time_s();


    println!("Factorial number: {}\nTime taken: {} seconds", character_adding_result, (end_time-start_time));
}
