rust
use std::thread;

fn main() -> io::Result<()> {
    // perform init work here (connect to db, warm caches, etc.)

    let max_threads = thread::available_concurrency().map(|n| n.get()).unwrap_or(1);
    let pool = ThreadPool::init(max_threads);

    // use pool for the duration of the program
}
