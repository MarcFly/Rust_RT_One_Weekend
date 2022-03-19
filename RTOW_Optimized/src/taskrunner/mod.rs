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
    tasks: Arc<RwLock<Box<VecDeque<Message>>>>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let queue = Arc::new(RwLock::new(Box::new(VecDeque::new())));
        let move_q = Arc::clone(&queue);

        let thread_b = thread::spawn(move || loop {
            {
                let mut msg;
                {
                    let mut task = move_q.write().unwrap();
                    
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

        Worker {id, 
            thread: Some(thread_b), 
            tasks: queue,
        }
    }
}

pub struct Runner {
    threads: Vec<Worker>,
    last_add: usize,
}

impl Runner {
    pub fn new(size: usize) -> Runner {
        let num = std::thread::available_parallelism().unwrap().get();

        let mut threads = Vec::with_capacity(num);
        
        for v in 0..num {
            threads.push(Worker::new(v));
        }

        Runner { threads, last_add: 0 }
    }

    pub fn add_task<F>(&mut self, f: F)
    where F: FnOnce() + Send + Sync + 'static,
    {
        self.threads[self.last_add].tasks.write().unwrap().push_back(Message::NewJob(Box::new(f)));
        //self.arc_q.write().unwrap().push_back(Message::NewJob(Box::new(f)));
        self.last_add = if self.last_add < self.threads.len() - 1 { self.last_add + 1} else {0};
    }

    pub fn ocupancy(&self) {
        for t in &self.threads {
            eprintln!("Thread {} with {} tasks.", t.id, t.tasks.read().unwrap().len());
        }
    }
}

impl Drop for Runner {
    fn drop(&mut self) {
        {
            //let mut write_q = self.arc_q.write().unwrap();
            for t in &self.threads {
                //add_taskself.sender.send(Message::Terminate).unwrap();
                t.tasks.write().unwrap().push_back(Message::Terminate);
                //write_q.push_back(Message::Terminate);
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