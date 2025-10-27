mod fetcher;
mod process;
mod scraper;

use std::{path::PathBuf, process::exit, time};

use fetcher::{fetch_geckodriver, find_geckodriver};
use process::ManagedProcess;

#[tokio::main]
async fn main() {
    let driver: PathBuf;
    if let Some(driver_) = find_geckodriver() {
        driver = driver_;
    } else if let Some(driver_) = fetch_geckodriver() {
        driver = driver_;
    } else {
        eprintln!("Error: Could not fetch geckodriver! Exiting...");
        exit(1);
    }

    let mut proc = ManagedProcess::start(driver).unwrap();
    println!("Process started (PID: {:?})", proc.child.id());

    let c = scraper::Scraper::new(scraper::Credentials {
        username: "yassin.diab".to_string(),
        password: "11223344Yd".to_string(),
    }).await.unwrap();

    tokio::time::sleep(time::Duration::from_secs(30)).await;

    if proc.is_running() {
        println!("Killing Process");

        c.close().await.unwrap();
        proc.kill().await;
    }
}
