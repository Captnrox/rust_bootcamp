use std::io;
use std::env;

fn collatz_length(mut n: i32) -> u32 {
    let mut len: u32 = 0;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Declare num before the match statement
    let num: i32;

    // Ensure there is at least one argument
    if args.len() > 1 {
        match args[1].as_str() {
            "help" => {
                println!("Usage:");
                println!("1 - Enter a number via user input");
                println!("2 [number] - Provide a number as a CLI argument");
                return; // Exit the program after printing help
            }
            "1" => {
                let mut input = String::new();
                num = loop {
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
            }
            "2" => {
                if args.len() < 3 {
                    println!("Error: Missing number for option 2.");
                    return;
                }
                num = match args[2].trim().parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid input! Please enter a valid number.");
                        return;
                    }
                };
            }
            _ => {
                println!("Unknown argument: {}", args[1]);
                return;
            }
        }
    } else {
        println!("Error: No arguments provided. Use --help for usage.");
        return;
    }

    // Now `num` is accessible here
    let len = collatz_length(num);
    println!("The length for {} : {}", num, len);
}