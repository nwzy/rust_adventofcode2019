use std::fs;
use std::io;
use std::mem;

fn main() -> io::Result<()> {
    let data = fs::read_to_string("input.txt")?;
    let data_trimmed = data.trim();
    let mut vector_data: Vec<i8> = Vec::new();
    let split_data = data_trimmed.split(",");
    for s in split_data {
        let s_int: i8 = s.parse().unwrap();
        vector_data.push(s_int);
    }

    for (i, element) in vector_data.iter().enumerate() {
        match element {
            1 => println!("index: {}, value: {}", i, element),
            2 => println!("index: {}, value: {}", i, element),
            99 => break,
            _ => continue,
        }
    }

    Ok(())
}