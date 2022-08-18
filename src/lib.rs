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
    /// Functions as the queue of jobs
    sender: mpsc::Sender<Job>,
}

/// Type alias for a trait object that holds the type of closure execute recives
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Creates a new ThreadPool.
    /// 
    /// Size is the number of threads in the pool.
    /// 
    /// # Returns
    /// On success this will return a new instance of a ThreadPool.
    /// On error this will return a PoolCreationError.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        // Return error if the pool was attempted to be created with no threads
        if size == 0 { 
            return Err(
                PoolCreationError::ZeroThreads(
                    "Attempted to create a pool with zero threads"
                )
            ) 
        }

        // Create a channel
        let (sender, receiver) = mpsc::channel();

        // Create a reciver that is contained inside a Mutex contained in a 
        // Arc, the mutex is so we dont run into any race conditions, and the Arc
        // is so we can share the receiver through multiple threads.
        let receiver = Arc::new(Mutex::new(receiver));

        // Create the vector of threads, with a preallocated size
        let mut workers = Vec::with_capacity(size);

        // Create the workers and push them onto the workers vector
        for i in 0..size {
            // thread::spawn cannot be used here since it expects to get some 
            // code that the thread should run immediately. But we just want to
            // create threads and have them wait until we create code later.
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        // All the workers have now been created and are waiting for jobs

        Ok(ThreadPool { workers, sender })
    }

    /// Sends the job you want to execute through the sender
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
        // Create a new job instance using the closure provided
        let job = Box::new(f);

        // Send the job down the channel where it will then be recived and
        // executed by one of the workers (threads)
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, recieiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let builder = Builder::new();
        let thread = match builder.spawn(move || loop {
            let job = recieiver
                                        // Block the current thread until we can 
                                        // aquire the mutex this mutex is so we
                                        // can access the receiver (this is behind
                                        // a mutex since channels can only have a
                                        // single consumer)
                                        .lock().unwrap()
                                        // Wait to receive a job from the channel
                                        // Jobs are sent down the channel from 
                                        // ThreadPool::execute
                                        .recv().unwrap();

            println!("Worker {id} got a job; executing");

            // Execute the job closure "extracted" above
            job();
        }) {
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