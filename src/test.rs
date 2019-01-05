use std::fmt::{self, Formatter, Display};
use std::println;

fn main() {

    let array = [1u32, 3, 3, 7];

    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &array);

    for i in array.iter() {
        println!("> {}", i);
    }
}