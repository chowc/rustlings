use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let m = Mutex::new(JobStatus { jobs_completed: 0 });
    let status = Arc::new(m);
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut lock = status_shared.lock().unwrap();
            lock.jobs_completed += 1;
            // status_shared.jobs_completed += 1;
        }
    });

    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}