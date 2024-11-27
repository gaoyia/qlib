use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>
}

type Task = Box<dyn FnOnce() + Send + 'static>;
impl Worker {
    fn new(id: usize,reciever: Arc<Mutex<Receiver<Task>>>) -> Self {

        let thread = thread::spawn(move || {
            loop {
                let receiver = reciever.lock()
                    .expect("Failed to grab the lock!")
                    .recv();
    
                match receiver {
                    Ok(task) => {
                        task();
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        });

        Self {
            id,
            thread:Some(thread)
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Task>>
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "Need at least 1 worker!");
        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));
        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i,Arc::clone(&reciever)));
        }

        Self { 
            workers,
            sender:Some(sender)
         }
    }
    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(job);

        self.sender
            .as_ref()
            .expect("Cannot execute the job!")
            .send(job)
            .expect("Failed to send the job to workers!");
    }
    
    pub fn join(&mut self) {
        if let Some(sender) = self.sender.take() {
            drop(sender);
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap_or_else(|_| panic!("Failed to join the thread {}", worker.id));
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.join();
    }
}