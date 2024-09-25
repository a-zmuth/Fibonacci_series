use std::io;

fn fibonacci(n: u128) -> u128 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    loop {
        println!("Input the nth term for the fibonacci series");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input: u128 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            };

            println!("Fibonacci series up to {} terms:", input);
            for i in 0..input {
            println!("{}", fibonacci(i));
        };
        break; 
    }  
}

/*fn fibonacci(n: u128) -> u128 {
    if n == 0 {
        return 0;
    } else if n == 1 {
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

fn main() {
    let n = 100; 
    println!("The {}th Fibonacci number is: {}", n, fibonacci(n));
}*/
