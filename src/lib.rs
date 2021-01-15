use std::thread;
use std::time::Instant;

use crossbeam::channel::{
    Sender,
    Receiver,
    unbounded
};
use uuid::Uuid;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

#[allow(dead_code)]
struct Worker {
    id: Uuid,
    thread: Option<thread::JoinHandle<()>>,
    created: Instant
}

impl Worker {
    fn new(receiver: Receiver<Message>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            id: Uuid::new_v4(),
            thread: Some(thread),
            created: Instant::now()
        }
    }
}

/// Thread pool of workers awaiting execution orders.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use std::sync::atomic::{ Ordering, AtomicBool };
    /// use threaded::ThreadPool;
    ///
    /// let num_workers = 2;
    /// let tp = ThreadPool::new(num_workers);
    ///
    /// assert_eq!(tp.capacity(), num_workers);
    ///
    /// let has_executed = Arc::new(AtomicBool::new(false));
    /// {
    ///     let has_executed = has_executed.clone();
    ///     tp.execute(move || {
    ///         has_executed.swap(true, Ordering::SeqCst);
    ///     });
    /// }
    ///
    /// drop(tp); // block main thread until execute finishes (uses handle.join())
    ///
    /// assert_eq!(has_executed, true);
    /// ```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // create crossbeam crate channel of unbounded capacity
        let (sender, receiver) = unbounded();

        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(receiver.clone()));
        }

        ThreadPool { workers, sender }
    }

    /// Capacity of thread pool (number of workers).
    pub fn capacity(&self) -> usize {
        self.workers.len()
    }

    /// Execute function/closure using worker from thread pool.
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // sending terminate to all workers
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // joining worker threads
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;
    use std::sync::atomic::{
        Ordering,
        AtomicBool
    };

    #[test]
    fn executes_spsc_job() {
        let tp = ThreadPool::new(1);
        let executed = Arc::new(AtomicBool::new(false));
        {
            let executed = executed.clone();
            tp.execute(move || {
                executed.swap(true, Ordering::SeqCst);
            });
        }
        drop(tp);
        assert_eq!(executed.load(Ordering::SeqCst), true);
    }

    #[test]
    fn executes_spmc_jobs() {
        // fixme - verify jobs run in parallel (worker id, overlap, etc)
        let tp = ThreadPool::new(2);
        let job1_executed = Arc::new(AtomicBool::new(false));
        let job2_executed = Arc::new(AtomicBool::new(false));
        {
            let job1_executed = job1_executed.clone();
            tp.execute(move || {
                job1_executed.swap(true, Ordering::SeqCst);
            });
        }
        {
            let job2_executed = job2_executed.clone();
            tp.execute(move || {
                job2_executed.swap(true, Ordering::SeqCst);
            });
        }
        drop(tp);
        assert_eq!(job1_executed.load(Ordering::SeqCst), true);
        assert_eq!(job2_executed.load(Ordering::SeqCst), true);
    }

    #[test]
    fn thread_pool_capacity_eq_num_of_workers() {
        let size = 2;
        let tp = ThreadPool::new(size);
        let expected = size;
        assert_eq!(tp.capacity(), expected);
    }
}
