fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 255);
  let (x, y, z) = tup;
  println!("x: {}, y: {}, z: {}", x, y, z);
  let first = tup.0;
  let second = tup.1;
  let third = tup.2;
  println!("first: {}, second: {}, third: {}", first, second, third);

  let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
  let month = months[0];
  println!("month: {}", month);
}
