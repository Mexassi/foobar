enum Colors {
  Red,
  Green,
  Blue,
  Purple,
}

fn main() {
  let mut color = Colors::Purple;
  println!("{}", value_in_hex(color));
  color = Colors::Red;
  println!("{}", value_in_hex(color));
  color = Colors::Green;
  println!("{}", value_in_hex(color));
  color = Colors::Blue;
  println!("{}", value_in_hex(color));
  dice_roll_fn();
}

fn value_in_hex(color: Colors) -> String {
  match color {
    Colors::Red => "FF0000".to_string(),
    Colors::Green => "00FF00".to_string(),
    Colors::Blue => "0000FF".to_string(),
    Colors::Purple => "800080".to_string(),
  }
}

fn dice_roll_fn() {
  let dice_roll = 9;
  match dice_roll {
      3 => add_fancy_hat(),
      7 => remove_fancy_hat(),
      // _ => reroll(),
      other => walk(other),
  }

  fn add_fancy_hat() {
    println!("Adding fancy hat");
  }
  fn remove_fancy_hat() {
    println!("Removing fancy hat");
  }
  // fn reroll() {
  //   println!("Rerolling");
  // }
  fn walk(value: u8) {
    println!("Walking {}", value);
  }
}
