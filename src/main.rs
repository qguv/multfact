extern crate rand;

use std::io;        // io::stdout()
use std::io::Write; // io::stdout().flush()
use rand::Rng;

static HIGHEST_MULTIPLICAND: i16 = 12;
static CHANCE_OF_DIVISION_PROBLEM: u32 = /* one in */ 3;
static CHANCE_OF_NEGATIVE_PROBLEM: u32 = /* one in */ 4;
static CHANCE_OF_DOUBLE_NEGATIVE: u32 = /* one in */ 2;

fn main() {
    loop {

        // mutable because rng access consumes entropy
        let mut rng = rand::thread_rng();

        let mut a = rng.gen_range(1, HIGHEST_MULTIPLICAND + 1);
        let mut b = rng.gen_range(1, HIGHEST_MULTIPLICAND + 1);

        let is_division_problem = rng.gen_weighted_bool(CHANCE_OF_DIVISION_PROBLEM);

        let is_negative_problem = rng.gen_weighted_bool(CHANCE_OF_NEGATIVE_PROBLEM);
        if is_negative_problem {
            let is_double_negative_problem = rng.gen_weighted_bool(CHANCE_OF_DOUBLE_NEGATIVE);
            if is_double_negative_problem {
                a = -a;
                b = -b;
            } else if rng.gen_weighted_bool(2) {
                a = -a;
            } else {
                b = -b;
            }
        }

        loop {

            if is_division_problem {
                print!("{} ÷ {} = ", a * b, a);
            } else {
                print!("{} × {} = ", a, b);
            }
            io::stdout().flush().unwrap();

            let mut guess = String::new();
            io::stdin().read_line(&mut guess).unwrap();

            let guess: i16 = match guess.trim().parse() {
                Ok(n)   => n,
                Err(_)  => {
                    println!("Huh?");
                    continue;
                },
            };

            let result = if is_division_problem { b } else { a * b };
            if guess == result {
                break;
            } else {
                println!("Try again.");
            }
        }
    }
}
