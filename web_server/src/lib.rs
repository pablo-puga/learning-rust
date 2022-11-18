use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    //! Creates a new worker given its id and a receiver channel
    //!
    //! It also spanws a thread
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        // If the operating system can't create a thread because there
        // aren't enough system resources, thread::spawn will panic.
        // That will cause our whole server to panic, even though the
        // creation of some threads might succeed. For simplicity's sake,
        // this behavior is fine, but in a production thread pool
        // implementation, you'd likely want to use std::thread::Builder
        // and its spawn method that returns Result instead.
        let thread = thread::spawn(move || loop {
            // Acquiring a lock might fail if the mutex is in a poisoned state,
            // which can happen if some other thread panicked while holding the
            // lock rather than releasing the lock.
            let message = receiver.lock().expect("Unable to adcquire lock!").recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                },
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the numbers of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker);
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    /// Adds the given closure to the jobs queue of the threadpool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
