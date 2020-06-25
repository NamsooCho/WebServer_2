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
    // to unlock the mutex, it needs to separate the lifetime of 'receiver' and 'message'
    fn get_message(id: u16, receiver: &SharedReceiver) -> Option<Message> {
        let mutex = if let Ok(mutex) = receiver.lock() {
            mutex
        } else {
            // how do I handle this kind of errors?
            eprintln!("[tread {}] fail to unwrap the mutex.", id);
            return None;
        };

        let message = if let Ok(message) = mutex.recv() {
            message
        } else {
            eprintln!("[thread {}] fail to receive the message.", id);
            return None;
        };

        Some(message)
    }
    thread::spawn(move || {
        loop {
            let message = if let Some(message) = get_message(id, &receiver) {
                message
            } else {
                continue
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


