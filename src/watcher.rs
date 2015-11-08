use std::error;
use std::thread;
use std::time::Duration;

pub trait Retriever<T>: Send {
    fn retrieve(&self) -> Result<Vec<T>, Box<error::Error>>;
}

pub trait EventHandler<T>: Send {
    fn on_data(&mut self, data: Vec<T>) -> Result<(), Box<error::Error>>;
}

pub struct Watcher<T> {
    retriever: Box<Retriever<T>>,
    handler: Box<EventHandler<T>>,
    interval: Duration,
}

impl<T: 'static> Watcher<T> {
    pub fn new(retriever: Box<Retriever<T>>, handler: Box<EventHandler<T>>, interval: Duration) -> Self {
        Watcher {
            retriever: retriever,
            handler: handler,
            interval: interval,
        }
    }

    pub fn watch(mut self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            loop {
                let result = self.retriever.retrieve().and_then(|data| self.handler.on_data(data));
                if let Err(err) = result {
                    println!("{}", err);
                }

                thread::sleep(self.interval);
            }
        })
    }
}
