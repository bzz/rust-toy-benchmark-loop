#![feature(test)]

mod lib;

fn main() {
    //lib::summ_iter();
    let sum = lib::summ_iter();
    println!("Sum: {}", sum);
}
