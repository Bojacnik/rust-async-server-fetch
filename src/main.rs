mod server_crasher;

use server_crasher::start;
use std::io::stdin;
use std::panic;

#[tokio::main]

async fn main() {
    
    loop {
        let result = panic::catch_unwind(|| {
            let mut input_string = String::new();
            let mut input_threads = String::new();

            println!("Enter URL: ");
            stdin().read_line(&mut input_string).expect("Failed to read line");
            let url = input_string.trim();

            println!("Enter number of threads: ");
            stdin().read_line(&mut input_threads).expect("Failed to read line");
            let input_threads = match input_threads.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => panic!("Not a valid u32"),
        };

            let url = format!("{}?response_type=json", url);
            println!("{}", url);

            start(input_threads, input_string);
        });

        if let Err(_) = result {
            println!("Bad Input");
        }
    }

}

