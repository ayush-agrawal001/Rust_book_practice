use std::{sync::mpsc::Receiver, time::Duration};

use trpl::{block_on, join, spawn_task, sleep, channel};



pub fn async_and_concurrency () {
    
    // Random same as threads 
    block_on(async {
        spawn_task(async {
            for i in 0..10 {
                println!("{i} from the spawned async");
                sleep(Duration::from_millis(100)).await;
                }
            }
        );

        for i in 0..5 {
            println!("{i} from the main async");
            sleep(Duration::from_millis(100)).await;
        }

        // Not random due to the join handle

        let fut_1 = async {
            for i in 6..11 {
            println!("{i} from the main async");
            sleep(Duration::from_millis(100)).await;
            };
        };

        let fut_2 = async {
            for i in 10..16 {
            println!("{i} from the main async");
            sleep(Duration::from_millis(100)).await;
            };
        };
        join(fut_1, fut_2).await;

    });

    masseging_in_async();
}


fn masseging_in_async () {
    let (tx, mut rx) = channel();

    block_on(async {
        let val = String::from("hi");
        tx.send(val).unwrap();


        let recieved = rx.recv().await.unwrap();

        println!("{recieved}");
    });

    block_on(async {
        let (tx1, mut rx2) = channel();
        
        // the tx and rx operations in their own async blocks to make the sleep operation work
        // Instead of reciving the messages single time.

        let tx1_o_1 = tx1.clone();

        let tx1_o_1_fut = async move { 
            // We need the move keyword here because we want to drop tx1 here
            // because if we dont drop it the program will not close, and we need to do it with ctrl-c
            let vals = vec![
                String::from("Hello"),
                String::from("Hi"),
                String::from("Namaste"),
            ];
        
            for val in vals {
                tx1_o_1.send(val).unwrap();
                sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(val) = rx2.recv().await {
                println!("{val}");
            }
        };

        // Making multiple producers
        let tx1_2_fut = async move{ 
            let vals = vec![
                String::from("Hello_2"),
                String::from("Hi_2"),
                String::from("Namaste_2"),
            ];
        
            for val in vals {
                tx1.send(val).unwrap();
                sleep(Duration::from_millis(500)).await;
            }
        };

        // join(tx1_o_1_fut, rx_fut).await;
        join!(tx1_o_1_fut, tx1_2_fut, rx_fut);
    })

}