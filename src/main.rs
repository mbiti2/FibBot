use std::env::args;

use get_pr::get_pr_body;

mod get_pr;
mod parse_numbers;

fn main() {

    get_pr_body(1);

    let result = get_pr_body(0);
    println!("{:?}", result);
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
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_0() {
        assert_eq!(fib(0), 0);
    }

    #[test]
    fn test_fib_1() {
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn test_fib_2() {
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn test_fib_5() {
        assert_eq!(fib(5), 5);
    }

    #[test]
    fn test_fib_10() {
        assert_eq!(fib(10), 55);
    }

    #[test]
    fn test_fib_large() {
        assert_eq!(fib(50), 12586269025); 
    }
}
