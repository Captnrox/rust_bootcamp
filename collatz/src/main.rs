use std::io;

fn collatz_length(mut n: i32) -> u32 {
    let mut len: u32 = 0;
    while n > 1 {
        n = if n%2 == 0 {n/2} else {3*n +1};
        len += 1;
    }

    len

}

fn main() {
    let mut input = String::new();

    let num: i32 = loop {
        println!("Give me a number.");

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(n) => break n, 
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
                input.clear(); 
            }
        }
    };

    let len = collatz_length(num);

    println!{ "The length for {} : {}", num, len};
}