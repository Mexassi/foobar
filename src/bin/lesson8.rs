struct Player {
  name: String,
}

impl Player {
  fn play(&self, other: &Player) {
    println!("{} is playing against {}!", self.name, other.name);
  }
}

fn main() {
  let player = Player {
    name: String::from("John"),
  };
  let other = Player {
    name: String::from("Jane"),
  };
  player.play(&other);
}
