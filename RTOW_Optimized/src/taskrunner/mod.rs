use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use std::collections::VecDeque;

pub type Job = Box<dyn FnOnce() + Send + Sync + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, arc_q: Arc<RwLock<Box<VecDeque<Message>>>>) -> Worker {
        let thread_b = thread::spawn(move || loop {
            {
                let mut msg;
                {
                    let mut task = arc_q.write().unwrap();
                    
                    match task.pop_front() {
                        Some(smth) => msg = smth,
                        None => continue,
                    }
                    
                }

                match msg {
                    Message::NewJob(job) => {
                        job();
                    },
                    Message::Terminate => break,
                }
            }
        });

        Worker {id, thread: Some(thread_b)}
    }
}

pub struct Runner {
    threads: Vec<Worker>,
    arc_q: Arc<RwLock<Box<VecDeque<Message>>>>,
}

impl Runner {
    pub fn new(size: usize) -> Runner {
        let num = std::thread::available_parallelism().unwrap().get();

        let mut threads = Vec::with_capacity(num);
        
        let arc_q = Arc::new(RwLock::new(Box::new(VecDeque::new())));
        for v in 0..num {
            threads.push(Worker::new(v, Arc::clone(&arc_q)));
        }

        Runner { threads, arc_q}
    }

    pub fn add_task<F>(&self, f: F)
    where F: FnOnce() + Send + Sync + 'static,
    {
        self.arc_q.write().unwrap().push_back(Message::NewJob(Box::new(f)));
    }
}

impl Drop for Runner {
    fn drop(&mut self) {
        {
            let mut write_q = self.arc_q.write().unwrap();
            for _ in &self.threads {
                //add_taskself.sender.send(Message::Terminate).unwrap();
                write_q.push_back(Message::Terminate);
            }
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