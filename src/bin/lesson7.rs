fn main() {
  let mut book = Book {
    id: String::from("id-one"),
    title: String::from("The holographic Universe"),
    author: String::from("Michael Talbot"),
  };
  println!("Author: {}", book.author);
  println!("Title: {}", book.title);
  println!("ID: {}", book.id);

  book.id = String::from("id-two");
  println!("ID: {}", book.id);

  let book2 = Book {
    title: String::from("Hello World"),
    ..book
  };
  dbg!(&book2);
}

#[derive(Debug)]
struct Book {
  id: String,
  title: String,
  author: String,
}
