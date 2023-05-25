// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
#![allow(dead_code)]

pub fn greeting_from_lib() -> String {
    let message = String::from("Hello from lib");
    message
}

trait Shape {
    fn area(&self) -> f32;
    fn new(length: f32, width: f32, name: &'static str) -> Self;
    fn get_length(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_name(&self) -> &'static str;
    fn set_name(&mut self, name: &'static str);
}

#[derive(Debug, Clone, Copy)]

struct Rect {
    length: f32,
    width: f32,
    name: &'static str,
}

impl Rect {
    fn default() -> Self {
        Rect {
            length: 1_f32,
            width: 1f32,
            name: "default_name",
        }
    }
}

impl Shape for Rect {
    ///Associated function used to create a new shape, it's not a method cause it doesn't use &self
    fn new(length: f32, width: f32, name: &'static str) -> Self {
        Rect {
            length, //dont have to use length: length cause theyre the same name
            width,
            name,
        }
    }
    ///Area method
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_length(&mut self, length: f32) {
        //todo!()
        self.length = length;
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn get_name(&self) -> &'static str {
        self.name
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        //self.length == other.length && self.width == other.width && self.name == other.name
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

///let the string element be on the heapr
trait Shape2 {
    //fn new(length: f32, width: f32, name: String) -> Self;
    fn area(&self) -> f32;
    fn get_length(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: &str);
}
#[derive(Default, Debug, Clone)]
struct Rect2 {
    length: f32,
    width: f32,
    name: String, //on heap
}

impl Rect2 {
    fn default() -> Self {
        Rect2 {
            length: 1_f32,
            width: 1_f32,
            name: "default_name".to_owned(),
        }
    }
}

impl Shape2 for Rect2 {
    ///Area method
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_length(&mut self, length: f32) {
        //todo!()
        self.length = length;
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn get_name(&self) -> String {
        self.name.to_owned()
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}

///implement a From trait for Rect2 that takes a string slice with the format "length, width, name"
impl From<&str> for Rect2 {
    //Essentially we are supposed to tokenize a string and extract the 3 parts into respective variables,
    //then use them to constitute a new Rect2
    fn from(s: &str) -> Self {
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0f32,
        };
        let width = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0f32,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Rect2 {
            length,
            width,
            name: name.to_owned(),
        }
    }
}

//Implement into trait for Rect2
impl Into<String> for Rect2 {
    fn into(self) -> String {
        //Let's return a string template literal
        format!("My name is {} and my area is {}", self.name, self.area())
    }
}

pub fn run() {
    let rectangle1 = Rect {
        length: 2.4 as f32,
        width: 6.3,
        name: "Rectangle 1",
    };
    let mut rectangle2 = Rect::default();
    let rectangle3 = rectangle1.clone();
    let rectangle4 = Rect {
        name: "Rectangle 4",
        ..rectangle1
    };

    rectangle2.set_length(10f32);
    rectangle2.set_width(5f32);
    println!("rectangle 1 is {:?}", rectangle1);
    println!("rectangle 2 is {:#?}", rectangle2);
    println!("Area of rectangle 1 is {:.2}", rectangle1.area());
    assert_eq!(rectangle1, rectangle4);
    println!("If you can see this, your triangles are equal")
}

//Let's demand that our struct be created on the heap

pub fn run2() {
    let rectangle1 = Box::new(Rect2 {
        length: 12f32,
        width: 9f32,
        name: "Rectangle 1".to_owned(),
    });
    let rectangle2 = Rect2::from("20.0,30.0,Rectangle2");
    let rectangle3: Rect2 = "25.0,37.0,Rectangke3".into(); //not the into we defined

    let s: String = rectangle3.into(); //the into we defined, also note that it's a move, meaning rectangle3 can't be used again
    println!("About me: {}", s);

    println!("{:#?}", rectangle1);
    println!("Area = {}", rectangle1.area());
    // let ans = apply(add, 4, 5);
    // println!("{}", ans)
}

///Functions and Closures

//In Rust, functions have their own types

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn apply(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    f(x, y)
}

// fn applied(fx: fn(f: fn(i32, i32)->i32, i32, i32) -> i32, m:i32, n:i32) -> i32{
//     fx(f, m, n)
// }

pub fn run3() {
    let f = add;
    let x = 7;
    let y = 8;
    let z = apply(f, x, y);
    println!("The result of applying f to {} and {} is {}", x, y, z);
}

///let's define another function that handles straight line graph formula
///Assuminng that m, c and x have to be passed.
///Here you can use a normal function.
///Below, we have to use array slice as x, otherwise, we will need to specify a size.
fn straight_line_function(m: i32, c: i32, xses: &[i32]) -> Vec<(i32, i32)> {
    let mut output: Vec<(i32, i32)> = Vec::new(); //you could also use vec![] to bring in initial arguments
    for x in xses {
        let y = (m * x) + c;
        output.push((*x, y)) //here we have to dereference the borrowed x, to get the value
    }
    output
}

pub fn run6(){
    let mut x = 10;
    println!("x before change = {}", x);

    let y = &mut x; //y is a mutable reference to x
    let z: *const u32 = y; //z is a constant pointer to y
    // let a  = y as *mut u32;
    let a:*mut u32 = y; //

    println!("y = {:?}", y); //expect value of x
    println!("z = {:?}", z); //expect address of x
    println!("a = {:?}", a); // expect same address as z

    *y = 11; //expect value in x to change
    println!("x after first change = {}", x);

    unsafe{
        *a = 12; //expect value in x to change
        assert!(x == 12);
    };
    println!("x after second change = {}", x);

}


pub fn run7(){
    // Error handling
    // See slides for reference

    // panic!("Problem. You called panic.")

    let mut v = vec!["a","b","c"];
    let x = v.pop();
    println!("{}", x.expect("I expected a value from my verctor. You messed up."));

    // What if there is possibility of having no Some value

    match x {
        Some(value) => println!("Popped {}", value),
        None => println!("Your vector is empty")
    }
    // Compare above to:
    // explicitly indicating that want a vector of strnigs
    let mut v2: Vec<&str> = Vec::new();

    // let mut y2: &str = v2.pop().expect(msg: "Do not call pop on an empty Vector");

    // Exercise: How  can you ensure your program does not panic when you...
    let y2: &str = match v2.pop() {
        Some(val: &str) => val,
        None => "Empty vector",
    };
    println!("{}", y2);

    // let's use ? for Option
    let mut v3: Vec<i32> = vec![1,2,3];

    let mut plus_one: impl FnMut() -> Option<i32> = || -> Option<i32>{
        Some(v3.pop()? + 1);
    }

    println!("Plus one {}", plus_one().unwrap());
}
