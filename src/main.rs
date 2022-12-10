use tokio_cron_scheduler::{JobScheduler, JobToRun, Job};

#[tokio::main]
fn main() {
    let mut sched = JobScheduler::new();
    sched.add(Job::new("1/10 * * * * *", |uuid, l| {
        println!("I run every 10 seconds");
    }).await.unwrap());

    // 世界よ、こんにちは
    println!("Hello, world!");
}