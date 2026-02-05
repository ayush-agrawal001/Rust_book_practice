use std::{sync::mpsc, thread, time::Duration};


pub fn thread_messages () {

    let (tx , rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {

        let vals = vec![
            "1", "2", "3"
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {

        let vals = vec![
            "4", "5", "6"
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for received in rx {
        println!("{}", received);
    }

}