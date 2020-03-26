use std::mem;

fn analyse_slice(slice: &[i32]) {
  println!("first element of the slice is {}", slice[0]);
  println!("the slice has {} elements", slice.len());
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

#[allow(unused_assignments, unused_variables)]
fn main() {
  let logical: bool = true;

  let a_float: f64 = 1.0;
  let an_integer = 5i32; // suffix annotation

  let default_float = 1.0; // 'f64'
  let default_integer = 7; // '132'

  let mut inferred_type = 12; // type is inferred from next line
  inferred_type = 4294967296i64;

  let mut mutable = 12;
  mutable = 21;

  // can mutate value but not type
  // this is an error
  // mutable = true;

  // shadowing to overwrite a variable
  let mutable = true;

  // literals and operators
  println!("1 + 2 = {}", 1u32 + 2);
  println!("1 - 2 = {}", 1i32 - 2);

  println!("true AND false is {}", true && false);
  println!("true OR false is {}", true || false);
  println!("NOT true is {}", !true);

  println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // 1000
  println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
  println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
  println!("1 << 5 is {}", 1u32 << 5);
  println!("0x80 >> 2 is 0x{:x}", 0x80 >> 2);

  println!("One million is written as {}", 1_000_000u32);

  // tuples
  fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (val1, val2) = pair;
    
    (val2, val1)
  }

  let long_tuple = (1u8, 2u16, 3u32, 4u64,
                    -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64,
                    'a', true);

  println!("long tuple first value: {}", long_tuple.0);
  println!("long tuple second value: {}", long_tuple.1);

  let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

  println!("tuple of tuples: {:?}", tuple_of_tuples);

  // long Tuples cannot be printed
  // let very_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
  // println!("{:?}", very_long_tuple);
  // get a `std::fmt::Debug` is not implemented for... error

  let pair = (1, true);
  println!("pair is {:?}", pair);
  println!("the reversed pair is {:?}", reverse(pair));

  // use a comma for a one element tuple
  println!("a one element tuple: {:?}", (5u32,));
  println!("just an integer: {:?}", (5u32));

  let example_tuple = (1, "hello", 4.5, true);
  let (a, b, c, d) = example_tuple;
  println!("{:?} destructured is a: {:?}, b: {:?}, c: {:?}, d: {:?}",
    example_tuple, a, b, c, d);

  let matrix = Matrix(1.1, 1.2, 1.3, 1.4);
  println!("{:?}", matrix);

  // arrays and slices
  let xs: [i32; 5] = [1, 2, 3, 4, 5];
  let ys: [i32; 500] = [0; 500];

  println!("first element of the array is {}", xs[0]);
  println!("second element of the array is {}", xs[1]);

  println!("array size is {}", xs.len());

  println!("array occupies {} bytes", mem::size_of_val(&xs));

  println!("borrow the whole array as a slice");
  analyse_slice(&xs);

  println!("borrow a section of the array as a slice");
  analyse_slice(&ys[1 .. 4]);

  // compile error as this index is out of bounds
  // println!("{}", xs[5]);
}