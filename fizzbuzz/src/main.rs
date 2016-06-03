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
