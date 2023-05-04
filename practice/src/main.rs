mod folder;
use folder::*;
// use folder::{tell, folder_one, folder_two};

extern crate practice_lib;

fn main() {
    println!("Hello, world!");
    println!("{}", folder::tell());
    println!("{}", tell());
    println!("{}", folder_one::one());
    println!("{}", folder_two::two());
    println!("{}", practice_lib::greeting_from_lib());
}