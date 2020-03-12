use std::fmt;

#[allow(dead_code)] // not needed as Structure is used
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8
}

struct DisplayableStruct(i32);

impl fmt::Display for DisplayableStruct {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}, {}", self.0, self.1)
  }
}

#[derive(Debug)]
struct Point2D {
  x: f64,
  y: f64
}

impl fmt::Display for Point2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "x: {}, y: {}", self.x, self.y)
  }
}

#[derive(Debug)]
struct Complex {
  real: f32,
  imag: f32
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} + {}i", self.real, self.imag)
  }
}

fn main() {
  println!("Hello world!");
  println!("I'm a Rustacean");

  // string replacement
  println!("My name is {}", "Rusty");

  // positional arguments
  println!("{0} this is {1}. {1} this is {0}", "Foo", "Bar");

  // named arguments
  println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over");

  // specially formatted arguments
  println!("{} of {:b}", 1, 2);

  // pad-left issues avoided in Rust :)
  println!("{number:>width$}", number=1, width=6);

  // 007
  println!("My name is {1}, {0} {1}", "James", "Bond");

  // pi rounding to 3 places
  let pi = 3.141592;
  println!("Pi is roughly {:.3}", pi);

  // Debug
  println!("{:?} months in a year.", 12);

  // but now with added struct printing
  println!("Debug of `Deep(Structure(3))` - {:?}", Deep(Structure(3)));

  // pretty print
  let name = "John";
  let age = 44;
  let john = Person { name, age };

  println!("{:?}", john);
  println!("{:#?}", john);

  println!("Display of `DisplayableStruct(5)` - {}", DisplayableStruct(5));

  let minmax = MinMax(0, 14);

  println!("Compare structures:");
  println!("Display: {}", minmax);
  println!("Debug: {:?}", minmax);

  let big_range = MinMax(-100, 300);
  let small_range = MinMax(-3, 3);

  println!("The big range is {big} and the small range is {small}",
    small = small_range,
    big = big_range);

  let point = Point2D { x: 3.3, y: 7.2 };

  println!("Compare points:");
  println!("Display: {}", point);
  println!("Debug: {:?}", point);

  let complex = Complex { real: 3.3, imag: 7.2 };

  println!("Compare complex:");
  println!("Display: {}", complex);
  println!("Debug: {:?}", complex);
}