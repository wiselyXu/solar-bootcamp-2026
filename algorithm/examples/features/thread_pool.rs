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
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        // receiver 锁已释放 ， MutexGuard.
                        println!("worker {id} got a job; executing");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down."); // 没有任务就直接断掉了？
                        break;
                    }
                }
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
    sender: Option<mpsc::Sender<Job>>,
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

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    // 其实 spawn 中没有  &spawn
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    // drop 方法， 在panicking 时会被 调用， 如果 join后的 unwrap 也被调了， 那么 就会导致  2 次panic, 这个有点不太好。 生产中不这么用。
    fn drop(&mut self) {
        //     for worker in &mut self.workers { // 因为是要对  self 里有内容做修改（删除） ，所以加 &mut,
        //         worker.thread.join().unwrap();    // thread， 其实要拿出来， 即所有权要拿出来， 不止是修改数据  vec::drain() 它可 remove 掉元素
        //     }

        drop(self.sender.take()); // 
        for worker in &mut self.workers.drain(..) {
            println!("shutting down worker {} ", worker.id);
            worker.thread.join().unwrap();
        }

        // 即使这样能够将线程remove， 但由于 worker 中是 死循环， 所以主线程会一直被 block住， 需要发一个信号 给 worker， 让它跳出循环。
    }
}
