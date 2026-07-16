enum IPAddr {
  V4(u8, u8, u8, u8),
}

fn main() {
  // let four = IPAddress {
  //   kind: IPAddressKind::V4,
  //   address: String::from("127.0.0.1"),
  // };
  // let six = IPAddress {
  //   kind: IPAddressKind::V6,
  //   address: String::from("::1"),
  // };

  // println!("{:?}", four.address);
  // println!("{:?}", six.kind);
  let ip = IPAddr::V4(127, 0, 0, 1);
  match ip {
    IPAddr::V4(a, b, c, d) => {
      println!("{}.{}.{}.{}", a, b, c, d);
    }
  }
}

// #[derive(Debug)]
// enum IPAddressKind {
//   V4,
//   V6,
// }

// #[derive(Debug)]
// struct IPAddress {
//   kind: IPAddressKind,
//   address: String,
// }
