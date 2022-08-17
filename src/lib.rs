use std::{
    process,
    thread::{Builder, JoinHandle}, 
    sync::{mpsc, Arc, Mutex},
};

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroThreads(&'static str)
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    /// Creates a new ThreadPool.
    /// 
    /// Size is the number of threads in the pool.
    /// 
    /// # Returns
    /// On success this will return a new instance of a ThreadPool.
    /// On error this will return a PoolCreationError.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 { 
            return Err(
                PoolCreationError::ZeroThreads(
                    "Attempted to create a pool with zero threads"
                )
            ) 
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        // Create the vector of threads, with a preallocated size
        let mut workers = Vec::with_capacity(size);

        // Create the threads
        for i in 0..size {
            // thread::spawn cannot be used here since it expects to get some 
            // code that the thread should run immediately. But we just want to
            // create threads and have them wait until we create code later.
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        
        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        // FnOnce() since that is what the Thread::spawn function uses as its 
        // trait bound for the closure argument it takes in, and we will be 
        // passing this closure into Thread::spawn. Next we have Send as a trait
        // bound because we need to transfer the closure from one thread to 
        // another. Lastly we have 'static because we dont know how long the
        // thread will take to execute.
        F: FnOnce() + Send + 'static
    {

    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, recieiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let builder = Builder::new();
        let thread = match builder.spawn(|| { recieiver; }) {
            Ok(t) => t,
            Err(e) => {
                println!("Failed to create worker {}: {}", id, e);
                process::exit(1);
            }
        };

        Worker {
            id,
            thread,
        }
    }
}