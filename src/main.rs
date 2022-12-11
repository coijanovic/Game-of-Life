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
            if roll < CHANCE {
                field[i][j] = false;
            } else {
                field[i][j] = true;
            }
        }
    }
}

fn main() {
    println!("Size is {SIZE}");
    let mut field = [[true; SIZE]; SIZE];
    for i in 0..SIZE-1 {
        field[i][i] = false;
    }
    init_random_field(&mut field);
    print_field(&field);
}
