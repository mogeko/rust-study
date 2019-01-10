use std::io;

fn main() {
    println!("How many numbers do you want to print?");
    let mut n = String::new();
    io::stdin().read_line(&mut n)
        .expect("This is not a valid input");
    let n: u64 = n.trim().parse()
        .expect("This is not a valid input");

    let mut i = 1u64;
    while i <= n {
        print!("{} ", fibonacci(i));
        i = i + 1;
    }
    
    println!("");
}

fn fibonacci(n: u64) -> u64 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n -1) + fibonacci(n - 2);
    }
}
