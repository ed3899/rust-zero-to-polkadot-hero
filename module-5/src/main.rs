// We're using a special toolbox called "rayon" to do lots of work super fast, like having many helpers
use rayon::prelude::*;

// This is the main part of our program, like the start of a big counting game
fn main() {
    // We make a huge list of numbers from 0 to 9,999,999, like a giant pile of toy blocks
    let numbers: Vec<u64> = (0..10_000_000).collect();

    // We ask a team of helpers (rayon) to take each block, double it, and add them all up
    // "par_iter" means all helpers work at the same time, like a super speedy team
    let sum: u64 = numbers.par_iter().map(|x| x * 2).sum();

    // We show the final total, like counting all the doubled blocks together
    println!("Sum = {sum}");
}