// this is a simple hello world

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

  // debug a struct
  #[allow(dead_code)]
  #[derive(Debug)]
  struct Structure(i32);

  // println!("This struct is `{}`", Structure(3));

  // pi rounding to 3 places
  let pi = 3.141592;
  println!("Pi is roughly {:.3}", pi);

  // Debug
  #[derive(Debug)]
  struct Deep(Structure);

  println!("{:?} months in a year.", 12);

  // but now with added struct printing
  println!("Now {:?} will print", Deep(Structure(3)));
}