use std::{collections, fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, result, thread::{self, JoinHandle}, time::Duration};

mod list;
use list::ThreadPool;


fn main() {
    
    let listner =TcpListener::bind("127.0.0.1:3000").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listner.incoming().take(2) { 
        // Take is from the vec, we use it here to test gracefull shutdown;

        let stream = stream.unwrap();

        println!("Connection established");

        pool.execute(|| {
            handleconnection(stream);   
        })

    }
}

fn handleconnection(mut stream : TcpStream) {

    let buffer = BufReader::new(&stream);

    let http_heads : String = buffer
                    .lines()
                    .next()
                    .unwrap()
                    .unwrap();

    // println!(" {http_heads:#?} ");

    let (response_status, file_name) = match &http_heads[..] {
       "GET / HTTP/1.1" =>  ("HTTP/1.1 200 OK", "hello.html"),
       "GET /sleep HTTP/1.1" => { thread::sleep(Duration::from_secs(3)); ("HTTP/1.1 200 OK", "hello.html")},
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(file_name).unwrap();

    let len = content.len();

    let response = format!("{response_status}\r\nContent-Length: {len}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();

}
