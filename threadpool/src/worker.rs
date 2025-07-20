use crate::job::Job;
use log::{debug, info};
use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self},
};
pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
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
            }),
        }
    }

    pub fn join(self) {
        info!("Shutting down worker {}", self.id);
        self.thread.join().unwrap();
    }
}
