fn main() {
  // let condition = true;
  // let number = if condition { 42 } else { 0 };
  // println!("The number is {}", number);
  // let mut counter = 0;
  // let result = loop {
  //   counter += 1;
  //   if counter > 10 {
  //     break counter;
  //   }
  // };
  // println!("Result: {}", result);
  loop_labels();
}

fn loop_labels() {
  let mut counter = 0;
  let result = loop {
    counter += 1;
    println!("Counter: {}", counter);
    'inner: loop {
      counter += 1;
      println!("Inner counter: {}", counter);
      if counter > 10 {
        break 'inner;
      }
    }
    if counter > 20 {
      break counter;
    }
  };
  println!("Result: {}", result);
}
