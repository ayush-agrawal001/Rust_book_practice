use std::{thread, time::Duration};

mod messages;
mod mutexes_in_rust;


use messages::thread_messages;
use mutexes_in_rust::mutexes_and_arc;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} Print from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // If we define this after the main thread for loop, they will run simoultanously untill 4 

    for i in 1..5 {
        println!("{} print from the main thread", i );
        thread::sleep(Duration::from_millis(1));
    }

    // ////////////////
    let v = vec![1, 2, 3];
    let handle_2 = thread::spawn(move || {
        println!("{:?}", v);
    });
    // The move keyword in closur moves the ownership of v from main thread to spawned thread


    handle_2.join().unwrap();

    thread_messages();

    mutexes_and_arc();
}
