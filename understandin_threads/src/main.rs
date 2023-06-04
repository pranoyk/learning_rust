use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    //we need to call join in case we want to make sure that the spawned thread to finish before the main thread
    //this will force the main thread to wait till the spawned thread is finished
    //if this join handle is called before the for loop in main then the spawned thread runs before the code return after it
    handle.join().unwrap();
}