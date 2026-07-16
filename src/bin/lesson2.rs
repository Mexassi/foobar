fn main() {
  // mutability
  let mut x = 5;
  println!("The value of x is: {x}");
  x = 6;
  println!("The value of x is: {x}");

  // shadowing
  let x = 42;
  println!("The value of x is: {x}");
}
