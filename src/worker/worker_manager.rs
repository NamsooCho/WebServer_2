//! ref <https://doc.rust-lang.org/book/ch20-02-multithreaded.html>

use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::SendError;

use crate::worker::task::Task;

use super::worker::Worker;

pub enum Message {
    Job(Box<dyn Task>),
    Terminate,
}

pub struct WorkerManager {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl WorkerManager {
    pub fn new(worker_count: u16) -> WorkerManager {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers: Vec<Worker> = Vec::with_capacity(worker_count as usize);
        let mut workers_manager = WorkerManager { workers, sender };

        for i in 0..worker_count {
            workers_manager
                .workers
                .push(Worker::new(i, Arc::clone(&receiver)));
        }

        workers_manager
    }

    pub fn request(&self, task: Box<dyn Task>) -> Result<(), SendError<Message>> {
        self.sender.send(Message::Job(task))?;

        Ok(())
    }
}

impl Drop for WorkerManager {
    fn drop(&mut self) {
        println!("sending terminate message to all workers");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("shutting down worker: {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let worker_count = 16;
        let worker_manager = WorkerManager::new(16);
        assert_eq!(worker_manager.workers.len(), worker_count)
    }

    #[test]
    #[should_panic]
    fn test_request() {
        struct PanicTask();

        impl Task for PanicTask {
            fn execute(&mut self) {
                panic!();
            }
        }

        let worker_manager = WorkerManager::new(1);
        worker_manager.request(Box::new(PanicTask {}));
    }
}
