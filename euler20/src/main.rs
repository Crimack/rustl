/*n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!*/


// I don't know how to fix this
// u64 is far too small to hold 100!, causing overflows and general panic

// TODO: Fix for factorials > 20

fn factorial(num : u64) -> u64 {
    if num == 0 { 1 }
    else {
        num * factorial(num - 1)
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
    let factorial_number = 20;
    let factorial_result: u64 = factorial(factorial_number);
    let character_adding_result: i32 = value_counter(&factorial_result.to_string());

    println!("Factorial number: {}", character_adding_result);
}
