use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

pub struct ThreadPool{
    threads: Vec<thread::JoinHandle<()>>,
    sender: Sender<T>,
}

impl ThreadPool{
    pub fn new(s: usize) -> ThreadPool {
        let size = if s <=0 { 1 }else{ s };
        let mut threads = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        for _ in 0..size {
            threads.push(thread::spawn(|| {}));
        }
        return ThreadPool { threads, sender };
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {

    }
}

struct Job{

}