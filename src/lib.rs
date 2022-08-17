pub struct ThreadPool;

impl ThreadPool {
    pub fn new(threads: usize) -> ThreadPool {
        ThreadPool
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