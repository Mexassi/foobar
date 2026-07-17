use std::collections::HashMap;

fn main() {
  let mut _vector: Vec<i32> = Vec::new();
  _vector.push(2);
  for i in &_vector {
    println!("{}", i);
  }
  let _vector = vec![1,2,3,4];
  let second = _vector.get(0);
  if let Some(second) = second {
    println!("{}", second);
  }

  let mut map = HashMap::new();
  map.insert("John", 20);
  let found = map.get("Johns");
  if let Some(age) = found {
    println!("{}", age);
  } else {
    panic!("ops")
  }
}
