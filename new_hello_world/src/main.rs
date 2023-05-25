mod greetings;
use greetings::{default_greeting, spanish, french};

mod how_to_hold_data;

use how_to_hold_data::derived::user_defined::Comp;
use how_to_hold_data::primitive::scalar::compare;
use how_to_hold_data::primitive::compound::arrays::multiplier;

extern crate new_hello_world_lib;

fn main() {
    // println!("Hello, world!");
    // println!("{}", default_greeting());
    // println!("In French: {}", french::default_greeting());
    // println!("In Spanish: {}", spanish::default_greeting());
    // println!("From the lib: {}", new_hello_world_lib::greeting_from_lib());
    // how_to_hold_data::primitive::scalar::run();
    new_hello_world_lib::run7();

    // let ans = compare(4, 5, Comp::LessThan);
    // println!("{}",ans);

    // let my_array = [3.0, 1.3, 3.8];
    // let my_array2 = [3, 13, 38];
    // println!("{}", multiplier(&my_array));
    // println!("{}", multiplier(&[my_array2[0] as f64, my_array2[1] as f64, my_array2[2] as f64]));
}
