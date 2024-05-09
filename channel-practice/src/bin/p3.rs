use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx,rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for mut val in vals {
            val.push_str(":tx1");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for mut val in vals {
            val.push_str(":tx2");
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}
