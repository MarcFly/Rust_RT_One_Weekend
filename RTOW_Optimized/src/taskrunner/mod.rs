use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::mpsc;

use std::collections::VecDeque;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
    Starved(usize),
}


use std::sync::atomic::AtomicI32;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
    tasks: Arc<Mutex<Box<VecDeque<Message>>>>,
}

impl Worker {
    fn new(id: usize, safe_send: mpsc::Sender<Message>) -> Worker {
        let queue = Arc::new(Mutex::new(Box::new(VecDeque::new())));
        let move_q = Arc::clone(&queue);

        let thread_b = thread::spawn(move || loop {
            {
                let mut msg;
                {
                    let mut ret: Option<Message> = { 
                        let mut task = move_q.lock().unwrap();
                        task.pop_front()
                    };

                    match ret {
                        Some(smth) => msg = smth,
                        None => {safe_send.send(Message::Starved(id)); sleep_ms(1); continue},
                    }

                }

                match msg {
                    Message::NewJob(job) => {
                        job();
                    },
                    Message::Terminate => break,
                    _ => ()
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
    receiver: mpsc::Receiver<Message>,
}

impl Runner {
    pub fn new(size: usize) -> Runner {

        let mut threads = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        for v in 0..size {
            threads.push(Worker::new(v, sender.clone()));
        }

        Runner { threads, last_add: 0, receiver}
    }

    pub fn add_task<F>(&mut self, f: F)
    where F: FnOnce() + Send + 'static,
    {
        //let mut least_num = i32::MAX;
        //let mut idx_least = 0;
        //for t in &self.threads {
        //    let len = t.tasks.read().unwrap().len() as i32;
        //    least_num = if least_num > len { idx_least = t.id; len} else {least_num};
        //}

        self.threads[self.last_add].tasks.lock().unwrap().push_back(Message::NewJob(Box::new(f)));
        self.last_add = if self.last_add < self.threads.len() - 1 { self.last_add + 1} else {0};

        
    }

    pub fn ocupancy(&self) {
        for t in &self.threads {
            eprintln!("Thread {} with {} tasks.", t.id, t.tasks.lock().unwrap().len());
        }
    }

    pub fn wait_all(&self) {
        let mut checker: Vec<bool> = Vec::new();
        checker.resize(self.threads.len(), false);
        let mut check_v = false;
        loop {
            
            loop {
                match self.receiver.try_recv() {
                    Ok(msg) => match msg {
                        Message::Starved(idx) => {
                        if(checker[idx]) {break};
                        checker[idx] = true;
                        },
                        _ => (),
                    },
                    TryRecvErr => break,
                    _ => (),
                }
            }

            for b in &checker {
                check_v = *b;
                if !check_v {break}
            }

            if check_v {break}
        }
    }

    pub fn join_all(&mut self) {
        {
            //let mut write_q = self.arc_q.write().unwrap();
            for t in &self.threads {
                //add_taskself.sender.send(Message::Terminate).unwrap();
                t.tasks.lock().unwrap().push_back(Message::Terminate);
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

impl Drop for Runner {
    fn drop(&mut self) {
        {
            //let mut write_q = self.arc_q.write().unwrap();
            for t in &self.threads {
                //add_taskself.sender.send(Message::Terminate).unwrap();
                t.tasks.lock().unwrap().push_back(Message::Terminate);
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


use std::thread::sleep_ms;

use simple_stopwatch::Stopwatch;

#[test]
fn sleep_test() {
    let mut t1 = Stopwatch::start_new();
    eprintln!("Sleep 1000 times 100ms in 12 threads...");
    {
        let mut tr = Runner::new(12);
        for i in 0..120 {
            tr.add_task(move || {
                sleep_ms(100);
            });
        }
    }
    eprintln!("Took {} ms\n", t1.ms());

    t1.restart();
    eprintln!("Sleep 120 times 100ms in 12 threads...");
    {
        let mut tr = Runner::new(1);
        for i in 0..120 {
            tr.add_task(move || {
                sleep_ms(100);
            });
        }
    }
    eprintln!("Took {} ms\n", t1.ms());

    t1.restart();
    eprintln!("Sleep 240 times 200ms in 12 threads...");
    {
        let mut tr = Runner::new(24);
        for i in 0..240 {
            tr.add_task(move || {
                sleep_ms(100);
            });
        }
    }
    eprintln!("Took {} ms\n", t1.ms());
}