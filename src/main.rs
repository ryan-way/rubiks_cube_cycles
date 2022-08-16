#![feature(test)]

extern crate test;

mod transforms;
mod rubiks;
mod algorithms;

use algorithms::*;
use rubiks::*;

fn run_repeated_test<F: Fn(&mut RubiksCube3x3)-> ()>(s: &str, f: F) {
    let mut rc = RubiksCube3x3::new();

    let mut count = 0;

    loop {
        f(&mut rc);

        count += 1;

        if rc.solved() {
            break;
        }
    }

    println!("{} took: {}", s, count);
}

fn main() {
    run_repeated_test("One of everthing", one_of_everything);
    run_repeated_test("One of everthing prime", one_of_everything_prime);
    run_repeated_test("RU", r_u);
    run_repeated_test("One of everything really", one_of_everything_really);
    run_repeated_test("Basic Move", basic_move);
}
