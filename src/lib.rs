use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::io::{self, Write};

type Job = Box<dyn FnOnce() + Send + 'static>; //thread-safe (Send) heap-alloc (Box) closure that takes ownership of captured data (FnOnce) since threads may outlive calling scope ('static)

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "Thread count must be >0");
        
        let (sender, receiver) = mpsc::channel(); //threadpool sender, shared receiver reference
        let receiver = Arc::new(Mutex::new(receiver));
        
        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();

        ThreadPool { 
            workers, 
            sender: Some(sender),
        }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap(); //called by main thread. send job down channel to be received by thread in pool
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); //.take() is necessary to close the channel so threads can join

        for worker in self.workers.drain(..) {
			println!("Worker {}: joined to main thread", worker.id);
            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
	id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
			let mut stdout = io::stdout();
			writeln!(stdout, "Worker {}: waiting for job", id).unwrap();
            loop {
                let job = match receiver.lock().unwrap().recv() { //mutex locked, recv called, mutex unlocked, recv blocks
                    Ok(job) => job,
                    Err(_) => {
                        writeln!(stdout, "Worker {}: channel closed", id).unwrap();
                        break;
                    }
                };
                writeln!(stdout, "Worker {}: executing job", id).unwrap();
                job();
            }
        });
        Worker { id, thread }
    }
}