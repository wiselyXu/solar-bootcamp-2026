use std::thread::{self, Thread};

pub struct ThreadPool;
// {
//   //  threads: Vec<thread::JoinHandle<()>>,

// }

fn main() {
    println!("ThreadPool example running.");
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        ThreadPool
    }

    pub fn execute<F, T>(&self, f: F)
    // 其实 spawn 中没有  &spawn
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
    }
}
