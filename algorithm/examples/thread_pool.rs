use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self},
    },
    thread::{self},
};

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                // receiver 锁已释放 ， MutexGuard.
                println!("worker {id} got a job; executing");
                job();
            }
            //receiver;
        });

        Worker { id, thread }
    }
}

//struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

fn main() {
    println!("ThreadPool example running.");
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let new_receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&new_receiver)));
            // create somet threads and store them in the vector
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    // 其实 spawn 中没有  &spawn
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
