use std::thread;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn is_aeiou(x: &char) -> bool {
    "aeiou".chars().any(|y| y == *x)
}

fn is_weird_auo(x: &char) -> bool {
    "äüö".chars().any(|y| y == *x)
}

fn valid(line: &str) -> bool {
    line.chars().any(|c| is_aeiou(&c)) && line.chars().filter(is_weird_auo).fuse().nth(1).is_some()
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("plainCipher.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();
    
    for line in lines {
        let handle_threads = thread::spawn(move || {
            println!("{:?}", line);
        });
    }

    // Alternate way if you don't need the line number. More readable
    //let all_good = lines.map(|l| l.unwrap()).all(valid);
}
