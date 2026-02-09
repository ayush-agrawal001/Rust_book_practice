use std::{sync::{Arc, Mutex, mpsc}, thread::{JoinHandle, spawn}};


pub struct ThreadPool{ 
    workers : Vec<Worker>,
    sender : Option<mpsc::Sender<Job>>
}

type Job = Box<dyn FnOnce() + Send + 'static>; // Dynamic state 

struct Worker { // Worker doing a job to transfer closure to the thread
    id : usize,
    thread : JoinHandle<()>
}

impl Worker  {
    fn new(id : usize, receiver : Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv();

                match job {
                    Ok(job) => {
                        println!("Worker {id} job is done");
        
                        job();
                    },
                    Err(_) => {
                        println!("No connection from the worker, Shutting down");
                        break;
                    }
                }
            }
        } );
        Worker { id, thread }
    }
}

impl ThreadPool {
    
    pub fn new(size : usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        // Because we want to share it to multiple consumer
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender : Some(sender) }
    }

    pub fn execute<F>( &self, f : F )
    where F : FnOnce() + Send + 'static 
    // The above trait bound is refered from the spawn fn impl in thread.
     {
      let job = Box::new(f);
      self.sender.as_ref().unwrap().send(job).unwrap()  
    }

}

// To gracefully shutdown the system

impl Drop for ThreadPool {
    fn drop(&mut self) {
        
        drop(self.sender.take());
        for worker in self.workers.drain(..)  { // Drain creates all vec element, to .next()
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
