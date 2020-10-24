use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let data = fs::read_to_string("../day_1_problem_1/input.txt")
        .expect("Unable to read file");

    let mut table: Vec<i32> = Vec::new();

    for element in data.lines() {
        if let Ok(elem) = element.parse() {
            table.push(elem)
        };
    }

    let mut fuel_vec: Vec<i32> = Vec::new();

    fn fuel_by_mod(mass_: &i32) -> i32 {
        let f_mass = *mass_ as f32;
        let mass = (f_mass / 3_f32) - 2_f32;
        mass.trunc() as i32
    }

    fn fuel_recurse(input: i32) -> i32 {
        let num = fuel_by_mod(&input);
        match num {
            n if n <= 0 => 0,
            _ => num + fuel_recurse(num)
        }
    }

    for i in table {
        fuel_vec.push(fuel_recurse(i));
    };

    let total_vec: i32 = fuel_vec.iter().sum();

    let elapsed = now.elapsed();
    print!("Sum of fuel_vec: {}\nProgram took {:?} to run", total_vec, elapsed);
}
