use std::env::args;

fn main() {
    let args: Vec<String> = args().skip(1).collect();

    if args.is_empty() {
        println!("No arguments supplied.");
        return;
    } else if args.len() != 2 {
        println!("FibBot requires exactly two parameters.");
        return;
    }

    let enable_fib = args[0].to_lowercase() == "true";
    
    let max_threshold: usize = match args[1].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid max_threshold value. It must be an integer.");
            return;
        }
    };

    if enable_fib {
        println!("FibBot enabled successfully with max_threshold: {}", max_threshold);
        let fib_result = fib(max_threshold);
        println!("Fibonacci result: {}", fib_result);
    } else {
        println!("Fibonacci calculation is disabled.");
    }
}

// Fibonacci function
pub fn fib(n: usize) -> usize {
    let mut prev = 0;
    let mut curr = 1;
    for _ in 0..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    prev
}
