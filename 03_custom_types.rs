#[derive(Debug)]
struct Person<'a> {
  // 'a defines a lifetime
  name: &'a str,
  age: u8,
}

// a unit struct
struct Nil;

// a tuple struct
struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn main() {
  let name = "Peter";
  let age = 27;
  let peter = Person { age, name };

  println!("{:?}", peter);

  let point: Point = Point { x: 3, y: 2 };

  println!("point coordinates: ({}, {})", point.x, point.y);

  let bottom_right = Point { x: 5, ..point };

  println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

  let Point { x: top_edge, y: left_edge } = point;

  let _rectangle = Rectangle {
    top_left: Point { x: left_edge, y: top_edge },
    bottom_right: bottom_right,
  };

  let _nil = Nil;

  let pair = Pair(1, 0.1);

  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  let Pair(integer, decimal) = pair;

  println!("pair contains {:?} and {:?}", integer, decimal);

  println!("area of rectangle {:?} is {}", _rectangle, rect_area(&_rectangle));
}

fn rect_area(rectangle: &Rectangle) -> i32 {

  let length = rectangle.bottom_right.x - rectangle.top_left.x;
  let width = rectangle.top_left.y - rectangle.bottom_right.y;

  length * width
}