fn main() {
  another_function(42);
  foo();
  let x = return_this();
  println!("x: {}", x);
}

fn another_function(x: i32) {
  println!("Another function. {}", x);
}

fn foo() {
  let y = {
    // use shadowing just because we can
    let y = 42;
    y + 1
  };
  println!("y: {}", y);
}

fn return_this() -> i32 {
  42
}
