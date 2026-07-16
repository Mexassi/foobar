// References, Borrowing, Safety and Performance
fn main() {
  let name = String::from("Rust");
  // the reference is passed to the function, not the value this is called borrowing
  // this is called an immutable borrow, there can be many immutable borrows at a time
  let length = get_length(&name);
  println!("The length of '{}' is {}", name, length);
  // only 1 owner at the time
  let name2 = name;
  println!("{}", name2);
  // this is a mutable borrow, there can only be one mutable borrow at a time
  let mut name3 = String::from("Rust");
  append(&mut name3, " is awesome!");
  println!("{}", name3);

  let mut account = BankAccount::new(String::from("John"), 1000.0);
  println!("Account opened with balance: {}", account.balance());
  account.deposit(500.0);
  println!("Account balance after deposit: {}", account.balance());
  account.withdraw(200.0);
  println!("Account balance after withdraw: {}", account.balance());
}

fn get_length(s: &str) -> usize {
  s.len()
}

fn append(s: &mut String, suffix: &str) {
  s.push_str(suffix);
}

/*
 * This is a block comment
 */
struct BankAccount {
  name: String,
  balance: f64,
}

impl BankAccount {
  fn new(name: String, balance: f64) -> Self {
    Self { name, balance }
  }

  fn deposit(&mut self, amount: f64) {
    self.balance += amount;
  }

  fn withdraw(&mut self, amount: f64) {
    self.balance -= amount;
  }

  fn balance(&self) -> String {
    format!("{}: {:.2}", self.name, self.balance)
  }
}
