use std::{process::Output, thread::sleep, time::Duration};

use trpl::{Either, block_on, select, yield_now};

fn slow_emulator(name : &str, dur : u64) {
    sleep(Duration::from_millis(dur));
    println!("{name} took {dur} milliseconds");
}

pub fn using_yields() {
    block_on(async {

        let a = async {
            println!("'a' started.");
            slow_emulator("a", 30);
            yield_now().await;
            slow_emulator("a", 40);
            yield_now().await;
            slow_emulator("a", 50);
            yield_now().await;
            println!("'a' finished.");
        };
        
        let b = async {
            println!("'b' started.");
            slow_emulator("b", 30);
            yield_now().await;
            slow_emulator("b", 40);
            yield_now().await;
            slow_emulator("b", 50);
            yield_now().await;
            println!("'b' finished.");
        };

        select(a, b).await;
    });  
}


pub async fn timeout_with_async_abstraction<F : Future>(fut : F, max_time : Duration) -> Result<F::Output, Duration> {
    
    match trpl::select(fut, trpl::sleep(max_time)).await {
        Either::Left(output)  => Ok(output),
        Either::Right(_) => Err(max_time)
    }

}