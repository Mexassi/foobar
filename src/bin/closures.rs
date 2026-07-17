fn main() {
  fn add_one_v1(x: u32) -> u32 { x + 1 }
  let add_one_v2 = |x: u32| -> u32 { x + 1 };

  let result = add_one_v1(4);
  println!("{}", result);
  let result = add_one_v2(5);
  println!("{}", result);

  let mut list = vec![1, 2, 3];
  println!("Before defining closure: {list:?}");
  // the fact that this is a closure blows my mind
  let mut borrows_mutably = || list.push(7);

  borrows_mutably();
  println!("After calling closure: {list:?}");

}
