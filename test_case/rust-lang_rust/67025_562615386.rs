rust
use futures::executor::ThreadPool;
use std::sync::mpsc;
use std::time::Duration;

async fn wait(d: Duration) -> f32 {
    d.as_secs_f32()
}

async fn add_task(sender: mpsc::Sender<f32>, i: u64) {
//    let v = wait(Duration::from_millis(i)).await;
//    sender.send(v).unwrap();
    sender.send(wait(Duration::from_millis(i)).await).unwrap();
}

fn main() {
    let (sender, receiver) = mpsc::channel();
    let thread_pool = ThreadPool::new().unwrap();
    for i in (0..10).rev() {
        thread_pool.spawn_ok(add_task(sender.clone(), i * 100));
    }
    drop(sender);
    for v in receiver {
        println!("{}", v);
    }
}
