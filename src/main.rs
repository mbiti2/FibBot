fn main() {
    // println!("Fibonacci sequence!");
    // println!("enter a number:");

    // let mut num = String::new();
    // io::stdin()
    //     .read_line(&mut num)
    //     .expect("failed to read line");
    // let num: usize = num.trim().parse().expect("enter a number");

    // println!("The fibonacci sequence of {num} is;");
    
    //  let results: usize = fibo(num).into();
     
    //  for int in 0..=num {
    //      println! ( "fibonacci ({}) => {}", int, fibo(int));
    //     //  println!("fibonacci ({num}) => {results}");
    // }
    let result = fib(7);

    println!("the fibonacci of  is {}",  result)
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