use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use reqwest::Client;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn get() -> Result<()> {
    let client = Client::new();
    let url = "https://zipcloud.ibsnet.co.jp/api/search";
    let response = client
        .get(url)
        .query(&[("zipcode", "1000002")])
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