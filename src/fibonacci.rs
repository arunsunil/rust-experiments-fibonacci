pub fn fibonacci(n: u32) -> u128 {
    if n == 1 || n == 2 {
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

pub fn fibonacci_loop(limit: u32) {
    for n in 1..limit {
        println!("Fibonacci ({})\t: {}", n, fibonacci(n))
    }
}
