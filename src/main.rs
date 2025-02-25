use std::env::args;

fn main() {
    let args: Vec<String> = args().skip(1).collect();

    if args.is_empty(){
        println!("no arguments supplied");
        return;
    } else if args.len() == 1 {
        println!("Fibbot requires two parameters");
    } else if args.len() ==2  {
        let argument_2 = &args[1];
        println!("fiboot enabled succesfully with max_threshold: {}", argument_2);
    } else {
        println!("wrong number of arguments parsed")
    }
}

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