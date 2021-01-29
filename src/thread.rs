
use std::env;
use serde::{ 
    Deserialize,
    Serialize
};

use chrono;
use std::error::Error;
use std::{
    thread,
    time::{Duration, Instant},
};

use tokio;

#[tokio::main]
pub async fn taskSCH() {
    let taskStart = thread::spawn(|| {
    let wait_time = Duration::from_secs(5);

    // infinte loop that monitors every 5 secs
    loop{

        let start = Instant::now();
        eprintln!("Thread Spawning at {:?}", start);
        let aliveThread = thread::spawn(monitorStarter);
        aliveThread.join().expect("Thread A panicked");

        //let runtime = start.elapsed();
        //if let Some(remaining) = wait_time.checked_sub(runtime) {
        //    eprintln!("schedule slice has time left over; sleeping for {:?}", remaining);
        //    thread::sleep(remaining);
        //}
        }
    });
    taskStart.join().expect("Current thread panicked");


fn monitorStarter() {
    
    eprintln!("MONITORING {:?}", chrono::Local::now());
    thread::sleep(Duration::from_secs(10))
    }
}