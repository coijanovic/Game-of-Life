use rand::prelude::*;

const SIZE : usize = 48;
const CHANCE : f64 = 0.5;

/// Prints the field in my *custom aesthetic*
fn print_field(field: &[[bool; SIZE]; SIZE]) {
    for i in 0..SIZE-1 {
        for j in 0..SIZE-1 {
            if field[i][j] {
                print!("🌝")
            } else {
                print!("🌚")
            }
        }
        print!("\n")
    }
}

/// Initializes the field with random values according to CHANCE const
fn init_random_field(field: &mut [[bool; SIZE]; SIZE]) {
    let mut rng = rand::thread_rng();
    for i in 0..SIZE-1 {
        for j in 0..SIZE-1 {
            let roll : f64 = rng.gen();
            if roll > CHANCE {
                field[i][j] = false;
            } else {
                field[i][j] = true;
            }
        }
    }
}

/// Returns how many livig neighbors the poit coord has
fn get_living_neighbor_count(coord: (u32, u32), field: &[[bool; SIZE]; SIZE]) -> u8 {
    let mut count: u8 = 0;
    for i in coord.0-1..=coord.0+1 {
        for j in coord.1-1..=coord.1+1 {
            if (i == coord.0 && j == coord.1) || (i == coord.1 && j == coord.0) {
                continue
            }
            if field[i as usize][j as usize] {
                println!("Field[{i}][{j}] is {}", field[i as usize][j as usize]);
                count += 1;
            }
        }
    }
    return count;
}

/// Evolves the given field one step
fn step(field: &mut [[bool; SIZE]; SIZE]) {

}

fn main() {
    println!("Size is {SIZE}");
    let mut field = [[true; SIZE]; SIZE];
    for i in 0..SIZE-1 {
        field[i][i] = false;
    }
    init_random_field(&mut field);
    print_field(&field);
    let testcount: u8 = get_living_neighbor_count((1,5), &field);
    println!("Count: {testcount}");
}
