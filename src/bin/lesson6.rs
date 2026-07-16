fn main() {
  let mut s = String::from("hello");
  s.push_str(" world");
  println!("{}", s);

  // let two = &mut s;
  // modify_string(two);
  // println!("{}", two);
  let first = first_word(&s);
  println!("{}", first);

  let str: String = String::from("Hey mate!");
  let _ref = &str[0..3];
  println!("{}", _ref);
}

// fn modify_string(s: &mut String) {
//   s.push_str("!!");
// }

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }
  &s[..]
}
