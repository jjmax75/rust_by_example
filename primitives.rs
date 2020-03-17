#[allow(unused_variables, unused_assignments)]
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
}