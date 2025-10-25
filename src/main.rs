mod fetcher;
mod process;
use std::{path::PathBuf, process::exit};

use fetcher::{fetch_geckodriver, find_geckodriver};
use process::ManagedProcess;

fn main() {
    let driver: PathBuf;
    if let Some(driver_) = find_geckodriver() {
        driver = driver_;
    } else if let Some(driver_) = fetch_geckodriver() {
        driver = driver_;
    } else {
        eprintln!("Error: Could not fetch geckodriver! Exiting...");
        exit(1);
    }

    let mut proc = ManagedProcess::start(driver, &["--headless"]).unwrap();
    println!("Process started (PID: {:?})", proc.child.id());

    //do something...
    std::thread::sleep(std::time::Duration::from_secs(5));

    if proc.is_running() {
        println!("Killing Process");
        proc.kill().unwrap();
    }
}
