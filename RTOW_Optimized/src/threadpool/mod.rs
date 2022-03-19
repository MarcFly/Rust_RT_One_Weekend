use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

mod mt_vec;
pub enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, safe_rec: Arc<Mutex<mpsc::Receiver<Message>>>)
     -> Worker 
    {
        let thread_b = thread::spawn(move || loop {
            let message = safe_rec.lock().unwrap().recv().unwrap();
            // Lock access to the variable and check if there is somethign to receive
            
            match message {
                Message::NewJob(job) => {
                    //eprintln!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    //eprintln!("Worker {} to terminate.", id);
                    break;
                }
            }
            
        });
    
        Worker { id, thread: Some(thread_b) }
    }
}

use std::sync::mpsc;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let num = if(size > 0) {size} else {1};

        let (sender, receiver) = mpsc::channel(); // Create connection
        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(num);

        for v in 0..num {
            threads.push(Worker::new(v, Arc::clone(&receiver)));
        }
        
        ThreadPool { threads, sender }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static,
    // FnOnce() with () because it represents closure without parameters and no return aka (), done for simplification purposes
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap(); 
    }

    //pub fn wait(&self) {
    //    while(self.)
    //}
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send a terminate message to all workers just in case
        //eprintln!("Terminating all workers");
        for _ in &self.threads {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.threads {
            // Vectors is already a collection that can be iterated
            // just use it as mutable reference so that iterator is also mutable
            //eprintln!("Joining worker: {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            } 
        }
    }
}