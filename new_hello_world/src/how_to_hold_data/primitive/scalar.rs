use crate::how_to_hold_data::derived::user_defined::Comp;

pub fn run(){
      println!("Hello, from the scalar module.");
}

pub fn compare(x:i32, y:i32, comparison:Comp) -> bool{
      match comparison {
          Comp::Equals => x==y,
          Comp::LessThan => x<y,
          Comp::MoreThan => x>y
      }
}