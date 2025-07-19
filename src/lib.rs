use std::{sync::{mpsc, Arc, Mutex}, thread::{self}};
use log::{debug, info};
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        Worker {
            id,     
            thread: thread::spawn(move || {
                loop {
                    let job = receiver.lock().unwrap().recv();
                    match job {
                        Ok(job) => {
                            debug!("Worker {id} got a job; executing.");
                            job();
                        }
                        Err(_) => {
                            debug!("Worker {id} disconnected; shutting down.");
                            break;
                        }
                    }
                }
            })
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender:Option< mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender: Some(sender) }
    }

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

        for worker in self.workers.drain(..) {
            info!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}