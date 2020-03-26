use std::fmt::{self, Formatter, Display, Result};

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

impl Display for DisplayableStruct {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl Display for MinMax {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}, {}", self.0, self.1)
  }
}

#[derive(Debug)]
struct Point2D {
  x: f64,
  y: f64
}

impl Display for Point2D {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "x: {}, y: {}", self.x, self.y)
  }
}

#[derive(Debug)]
struct Complex {
  real: f32,
  imag: f32
}

impl Display for Complex {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{} + {}i", self.real, self.imag)
  }
}

struct List(Vec<i32>);

// leaving fmt:: here as an example
impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let vec = &self.0;

    write!(f, "[")?;

    for (index, v) in vec.iter().enumerate() {
      if index != 0 { write!(f, ", ",)?; }
      write!(f, "{}: {}", index, v)?;
    }

    write!(f, "]")
  }
}

#[derive(Debug)]
struct City {
  name: &'static str,
  lat: f32,
  lon: f32
}

impl Display for City {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
    let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

    write!(f, "{}: {:.3}°{} {:.3}°{}",
      self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
  }
}

#[derive(Debug)]
struct Color {
  red: u8,
  green: u8,
  blue: u8
}

impl Display for Color {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "RGB ({0:3}, {1:3}, {2:3}) 0x{0:02X}{1:02X}{2:02X}",
      self.red, self.green, self.blue)
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

  let v = List(vec![1, 2, 3]);

  println!("{}", v);

  let cities = [
    City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
    City { name: "Oslo", lat: 59.95, lon: 10.75 },
    City { name: "Vancouver", lat: 49.25, lon: -123.1 }
  ];

  for city in cities.iter() {
    println!("{}", *city);
  }

  for color in [
    Color { red: 128, green:255, blue: 90 },
    Color { red: 0, green: 3, blue: 254 },
    Color { red: 0, green: 0, blue: 0 }
  ].iter() {
    println!("{}", *color);
  }
}