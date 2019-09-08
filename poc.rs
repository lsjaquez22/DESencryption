use std::thread;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let fine_name = "plainCipher.txt";

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = File::open(fine_name).unwrap();

    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let string = line.unwrap();
        let v: Vec<&str> = string.split(',').collect();
        let plain = convert_bool(v[0]);
        let cipher = convert_bool(v[1]);
        let handle_threads = thread::spawn(move || {
            println!("{:?} converts to {:?}", plain, cipher);
        });
    }

    // Alternate way if you don't need the line number. More readable
    //let all_good = lines.map(|l| l.unwrap()).all(valid);
}

fn convert_bool(string: &str) -> [bool; 8] {
    let mut arr: [bool;8]=[false;8];
    for (i, a) in string.chars().enumerate() {
        if a == '1' {
            arr[i] = true;
        } else {
            arr[i] = false;
        }
    }
    return arr;
}