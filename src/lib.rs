use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Job>,
}

impl ThreadPool {
	pub fn new(size: usize) -> ThreadPool {
		assert!(size>0);
		
		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));
		
		let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
	}
	pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
		let job = Box::new(f);
        self.sender.send(job).unwrap();
	}
}

struct Worker {
	id: usize,
	thread: thread::JoinHandle<()>,
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		Worker {
			id, 
			thread: thread::spawn(move || loop {
				println!("Worker {id} executing");
				receiver.lock().expect("Mutex locked by other thread").recv().unwrap();
			})
		}
	}
}

type Job = Box<dyn FnOnce() + Send + 'static>;