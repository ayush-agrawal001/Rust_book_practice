
// Concurrency vs Parallelism
// Concurrency -> When an individual works on several different tasks before any of them is complete.
// Parallelism -> When different individual's work on only one task at same time.

mod concurrency_with_async;
mod async_yields;

use std::{env::args, time::Duration};
use trpl::{self, Either, Html, block_on, select};
use concurrency_with_async::async_and_concurrency;
use async_yields::{using_yields, timeout_with_async_abstraction};

fn main() {
    let args : Vec<String> = args().collect();

    
    let url_1 = &args[1];
    let url_2 = &args[2];

    block_on(
        async {
            let title_option_1 = page_title(url_1);
            let title_option_2 = page_title(url_2);
        
            let (url, maybe_title) = 
                match select(title_option_1, title_option_2).await {
                    Either::Left(left) => left,
                    Either::Right(right) => right,
                };
        
        
            match maybe_title {
                Some(title) => println!("The title from the {url} given is '{}' ", title),
                None => println!("No title in the url")
            }
        }
    );
    
    async_and_concurrency();

    using_yields();

    block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };
        match timeout_with_async_abstraction(slow, Duration::from_secs(2)).await {
            Ok(msg) => println!("Succeded with {msg}"),
            Err(duration) => println!("Failed after {} sec", duration.as_secs()) 
        }
    })
}

async fn page_title(url  : &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;

    let response_text = response.text().await;

    let title = Html::parse(&response_text)
                                    .select_first("title")
                                    .map(|title| title.inner_html());

    (url, title)
}