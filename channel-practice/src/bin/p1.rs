use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("Hi");
        //send() takes the ownership of msg variable
        tx.send(msg).unwrap();
    });

    //blocks the thread till a message is received or channel is closed
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
