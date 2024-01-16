use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("[{i}] from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("[{i}] from the main thread");
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}
