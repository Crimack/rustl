
use std::collections::HashMap;
use std::env::args;

fn factorial(current: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if cache.contains_key(&current) {
        return *cache.get(&current).unwrap();
    }
    if current == 1 {
        return 1;
    } else {
        let answer = current * factorial(current - 1, cache);
        cache.insert(current, answer);
        return answer;
    }
}

fn main() {

    let target: u64 = args()
                        .nth(1)
                        .expect("Expected an integer")
                        .parse::<u64>()
                        .unwrap();
    let mut cache = HashMap::new();

    let result = factorial(target, &mut cache);

    println!("Factorial {}: {}", target, result);
}
