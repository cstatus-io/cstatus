use std::{thread, time::Duration};

pub struct Worker;

impl Worker {
    pub fn start(&self) {
        println!("Creating worker ... ");
        thread::spawn(|| {
            for i in 1..30 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }
}
