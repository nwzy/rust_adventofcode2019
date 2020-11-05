use std::fs;
use std::io;
//use std::mem;
// unsure if you wanted to explore std::mem and that's why you used it,
// but it's not necessary to solve this problem

fn op1(val_1:usize,val_2:usize) -> usize {
    let val_3 = val_1 + val_2;
    return val_3;
}

fn op2(val_1:usize,val_2:usize) -> usize {
    let val_3 = val_1 * val_2;
    return val_3;
}

fn main() -> io::Result<()> {
    let data = fs::read_to_string("input.txt")?;
    let data_trimmed = data.trim();
    let mut vector_data: Vec<usize> = Vec::new();
    let split_data = data_trimmed.split(",");
    for s in split_data {
        let s_int: usize = s.parse().unwrap();
        vector_data.push(s_int);
    }
    let length = vector_data.len();
    println!("Vector is {} elements long", length);

// Setting 1202 Program Alarm State
    vector_data[1] = 12;
    vector_data[2] = 2;

    for i in 0..length {
// We want to look index 0,4,8,etc -- not just any element with a usable opcode
// Usable opcodes could be stored at index 2, for example
/*        match element {
            1 => println!("index: {}, value: {}", i, element),
            2 => println!("index: {}, value: {}", i, element),
            99 => break,
            _ => continue,

            }
 */
        let is_usable_index = i%4;
        if is_usable_index == 0 {
            if vector_data[i] == 99 {
                println!("{} is left at position 0 at end of program.", vector_data[0]);
                break;
            }
//            println!("Opcode is valid! {}", i);
            let pos_1 = vector_data[i+1];
            let pos_2 = vector_data[i+2];
            let pos_3 = vector_data[i+3];
            let val_1 = vector_data[pos_1];
            let val_2 = vector_data[pos_2];
//            println!("Val 1 is {}", val_1);
//            println!("Val 2 is {}", val_2);
            let mut new_val = 0;
            let opcode = vector_data[i];
            if opcode == 1 {
//                println!("New val is {}", new_val);
                new_val = op1(val_1,val_2);
//                println!("New val is {}", new_val);
//                println!("Hey, we're in");
                println!("Addition of {} and {} results in {}",val_1,val_2,new_val);
            }
            if opcode == 2 {
                new_val = op2(val_1,val_2);
                println!("Multiplication of {} and {} results in {}",val_1,val_2,new_val);
            }
            vector_data[pos_3] = new_val;
        }


    }
    Ok(())
}
