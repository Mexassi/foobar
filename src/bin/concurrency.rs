use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

 fn main() {
    let handle = thread::spawn(|| {
      for i in 1..10 {
          println!("hi number {i} from the spawned thread!");
          thread::sleep(Duration::from_millis(1));
      }
    });

    for i in 1..5 {
      println!("hi number {i} from the main thread!");
      thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    ownership_in_threads();

    channels();
 }

 fn ownership_in_threads() {
   let v = vec![1, 2, 3];

   let handle = thread::spawn(move || {
       println!("Here's a vector: {v:?}");
   });

   handle.join().unwrap();
 }

 fn channels() {
   // channels are multiple producer, single consumer they are used to communicate values across threads
   let (tx, rx) = mpsc::channel();

   thread::spawn(move || {
     let val = String::from("Ciao");
     tx.send(val).unwrap();
     tx.send(String::from("Foo")).unwrap();
   });

   let received = rx.recv().unwrap();
   let second_received = rx.recv().unwrap();
   println!("Received: {received}");
   println!("Second received: {second_received}");
 }
