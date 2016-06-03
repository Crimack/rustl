

fn main(){

    // "If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
    // Find the sum of all the multiples of 3 or 5 below 1000.";

    let mut sum: i32 = 0;

    for i in 0..1000 {
        if (i % 3 == 0) | (i % 5 == 0) {
            sum += i;
        }
    }

    println!("The answer is: {}", sum);

    // FizzBuzz

    for i in 0..100 {
        let result = if (i % 3 == 0) | (i % 5 == 0) {
            "FizzBuzz".to_string()
        } else if i % 3 == 0 {
            "Fizz".to_string()
        } else if i % 5 == 0 {
            "Buzz".to_string()
        } else {
            i.to_string()
        };
        println!("{}", result);

    }

    // Pattern matching fizzbuzz - found on SO but saved because it's pretty

    for i in 1..101 {
        match (i % 3, i % 5){
            (0, 0) => println!("Fizzbuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{}", i),
        }
    }

}
