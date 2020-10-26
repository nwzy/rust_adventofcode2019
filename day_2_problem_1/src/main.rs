use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let data = fs::read_to_string("input.txt")?;
    let data_trimmed = data.trim();
    let mut vector_data: Vec<i32> = Vec::new();
    let split_data = data_trimmed.split(",");
    for s in split_data {
        let s_int: i32 = s.parse().unwrap();
        vector_data.push(s_int);
    }

    println!("Vector input:\n {:?}\n", vector_data);

    std::mem::replace(&mut vector_data[1], 12);
    std::mem::replace(&mut vector_data[2], 2);

    println!("\nMinor adjustments. New vector input:\n {:?}\n", &mut vector_data);

    for i in 0..vector_data.len() {
        let element = vector_data[i];
        match element {
            1 => {
                let val_a = vector_data[i + 1];
                let val_b = vector_data[i + 2];
                println!("index: {}, found 1, replacing {} with {} at index {}",
                    i, vector_data[i + 3], val_a+val_b, i+3
                );
                std::mem::replace(&mut vector_data[i + 3], val_a+val_b);
            },
            2 => {
                let val_a = vector_data[i + 1];
                let val_b = vector_data[i + 2];
                println!("index: {}, found 2, replacing {} with {} at index {}",
                    i, vector_data[i + 3], val_a*val_b, i+3
                );
                std::mem::replace(&mut vector_data[i + 3], val_a*val_b);
            }
            99 => {
                println!("index: {}, value: 99, breaking", i);
                break;
            },
            _ => continue,
        }
    }

    println!("\nThe new vector is:\n {:?}\n", &mut vector_data);

    Ok(())
}

// shepmaster way of taking a file of ints as a string and converting the type
// to ints

// use std::fs;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let data = fs::read_to_string("input.txt")?;
//     let vector_data = data
//         .split(",")
//         .map(|s| s.trim().parse())
//         .collect::<Result<Vec<i8>, _>>()?;
//     Ok(())
// }

// Using indices to iterate so as to allow mutation of vector_data since you are
// not allowed to mutate a struct that is already borrowed

// for i in 0..vector_data.len() {
//     let element = vector_data[i];
//     match element {
//         1 => {
//             let val_a: i8 = vector_data[i + 1];
//             let val_b: i8 = vector_data[i + 2];
//             std::mem::replace(&mut vector_data[i + 3], val_a + val_b);
//         }
//         2 => println!("index: {}, value: {}", i, element),
//         99 => break,
//         _ => continue,
//     }