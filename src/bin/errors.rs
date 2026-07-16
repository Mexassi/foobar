fn main() {
  let result = divide(10, 0);
  println!("{:?}", result.err());
}

// enum Option<T> {
//   Some(T),
//   None,
// }

// enum Result<T, E> {
//   Ok(T),
//   Err(E),
// }

fn divide(a: i32, b: i32) -> Result<i32, String> {
  if b == 0 {
    Err("division by zero".to_string())
  } else {
    Ok(a / b)
  }
}
