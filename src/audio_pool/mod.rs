use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use once_cell::sync::Lazy;

use crate::write_log;
static END: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));
static POOL: Lazy<Arc<Mutex<Vec<JoinHandle<()>>>>> = Lazy::new(|| Arc::new(Mutex::new(vec![])));

static QUEUE: Lazy<Arc<Mutex<Vec<Box<dyn FnOnce() + Send + Sync>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(vec![])));

pub fn initialize(size: usize) -> Result<(), String> {
    let Ok(mut pool) = POOL.lock() else {
        return Err("failed lock pool".to_string());
    };

    for _i in 0..size {
        pool.push(thread::spawn(|| loop {
            let Ok(end) = END.lock() else {
                continue;
            };
            if *end {
                return;
            }
            if let Some(f) = dequeue() {
                f();
                continue;
            };
        }));
    }
    Ok(())
}

pub fn finalize() {
    let Ok(mut end) = END.lock() else {
        write_log(b"audio_pool::finalize: failed lock end flag\r\n");
        return;
    };
    *end = true;
    
    let Ok(mut pool) = POOL.lock() else {
        write_log(b"audio_pool::finalize: failed lock pool\r\n");
        return;
    };
    for handle in pool.drain(..) {
        handle.join().unwrap();
    }
}

pub fn enqueue<F>(f: F) -> Result<(), String>
where
    F: FnOnce() + Send + Sync + 'static,
{
    let Ok(mut queue) = QUEUE.lock() else {
        return Err("failed lock queue".to_string());
    };
    queue.push(Box::new(f));
    Ok(())
}

pub fn dequeue() -> Option<Box<dyn FnOnce() + Send + Sync>> {
    let Ok(mut queue) = QUEUE.lock() else {
        panic!("failed lock queue");
    };
    queue.pop()
}
