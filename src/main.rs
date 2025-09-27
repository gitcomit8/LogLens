use std::io::{self, BufRead};

fn main() {
    //handle to standard input stream
    let stdin = io::stdin();

    //Lock the handle for exclusive access
    for line in stdin.lock().lines() {
        //Each line is a Result<String,Error>
        //Use match to handle both Ok and Err
        match line {
            Ok(l) => {
                println!("{}", l);
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
}
