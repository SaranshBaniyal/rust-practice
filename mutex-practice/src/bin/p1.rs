use std::sync::Mutex;
fn main() {
    let m = Mutex::new(3);

    {
        let mut num = m.lock().unwrap();
        *num = 9;
    }
    //Mutex lock will be released here automatically

    println!("m = {:?}", m);
}
