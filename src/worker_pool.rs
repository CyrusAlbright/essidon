use std::thread;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

type Job = Box<dyn FnOnce() + Send + 'static>;
type MessageReceiver = Arc<Mutex<mpsc::Receiver<Message>>>;
type MessageSender = mpsc::Sender<Message>;
type Thread = thread::JoinHandle<()>;

enum Message {
	NewJob(Job),
	Terminate
}

pub struct WorkerPool {
	workers: Vec<Worker>,
	sender: MessageSender
}

impl WorkerPool {
	pub fn new(size: usize) -> WorkerPool {
		assert!(size > 0);

		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		WorkerPool { workers, sender }
	}

	pub fn execute<F>(&self, f: F) 
	where
		F: FnOnce() + Send + 'static,
	{
		let job = Box::new(f);

		self.sender.send(Message::NewJob(job)).unwrap();
	}
}

impl Drop for WorkerPool {
	fn drop(&mut self) {
		for _ in &self.workers {
			self.sender.send(Message::Terminate).unwrap();
		}

		for worker in &mut self.workers {
			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
	}
}

struct Worker {
	id: usize,
	thread: Option<Thread>
}

impl Worker {
	fn new(id: usize, receiver: MessageReceiver) -> Worker {
		let thread = thread::spawn(move || loop {
			let message = receiver.lock().unwrap().recv().unwrap();
			
			match message {
				Message::NewJob(job) => {
					job();
				},
				Message::Terminate => {
					break;
				}
			}
		});

		Worker { id, thread: Some(thread) }
	}
}

