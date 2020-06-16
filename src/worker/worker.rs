use std::sync::{Arc, mpsc, Mutex};
use std::thread;

use crate::worker::worker_manager::Message;

type SharedReceiver = Arc<Mutex<mpsc::Receiver<Message>>>;

pub struct Worker {
    pub id: u16,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: u16, receiver: SharedReceiver) -> Worker {
        let mut worker = Worker { id, thread: None };
        worker.thread = Some(make_thread(worker.id, receiver));

        worker
    }
}

fn make_thread(id: u16, receiver: SharedReceiver) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let mutex = if let Ok(mutex) = receiver.lock() {
                mutex
            } else {
                // how do I handle this kind of errors?
                eprintln!("[tread {}] fail to unwrap the mutex.", id);
                continue;
            };

            let message = if let Ok(message) = mutex.recv() {
                message
            } else {
                eprintln!("[thread {}] fail to receive the message.", id);
                continue;
            };

            match message {
                Message::Job(mut task) => {
                    task.execute();
                },
                Message::Terminate => break,
            }
        }
    })
}


