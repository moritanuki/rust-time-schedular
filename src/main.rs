use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use reqwest::Client;
use dotenv::dotenv;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn get() -> Result<()> {
    dotenv().ok();

    let client = Client::new();
    let url = env::var("TOKEN").unwrap();
    
    let response = client
        .get(url)
        .header("Authorization", env::var("TOKEN").unwrap())
        .send()
        .await?;
    
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}

fn main() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("0 * * * * *".parse().unwrap(), || {
        let _result = get();
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
  }