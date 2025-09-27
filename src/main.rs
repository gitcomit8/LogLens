use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::io::{self, BufRead};

#[derive(Debug, Deserialize)]
struct ParsedLine {
    //Option<String> is used because these fields might not be in
    //every log line
    level: Option<String>,
    message: Option<String>,

    //this macro tells serde to put all other fields
    //from the JSON object into this map
    #[serde(flatten)]
    rest: BTreeMap<String, Value>,
}
fn main() {
    //handle to standard input stream
    let stdin = io::stdin();

    //Lock the handle for exclusive access
    for line in stdin.lock().lines() {
        //Each line is a Result<String,Error>
        //Use match to handle both Ok and Err
        match line {
            Ok(l) => match serde_json::from_str::<ParsedLine>(&l) {
                Ok(parsed_line) => {
                    println!("{:?}", parsed_line);
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
}
