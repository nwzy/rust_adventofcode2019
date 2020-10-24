use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt")
        .expect("Unable to read file");

    let mut table: Vec<i32> = Vec::new();

    for element in data.lines() {
        if let Ok(elem) = element.parse() {
            table.push(elem)
        };
    }

    fn fuel_by_mod(mass_: &i32) -> i32 {
        let f_mass = *mass_ as f32;
        let mass = (f_mass / 3_f32) - 2_f32;
        mass.trunc() as i32
    }

    let total: i32 = table.iter().map(|&x| fuel_by_mod(&x)).sum();

    println!("Sum of required fuel: {}", &total);
}
