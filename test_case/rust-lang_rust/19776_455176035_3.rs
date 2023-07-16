rust
    use std::thread;
    const NTHREADS: i32 = 10;
    // This is the `main` thread
    fn main() {
        // Make a vector to hold the children which are spawned.
        let mut children = vec![];
        for i in 0..NTHREADS {
            // Spin up another thread
            children.push(thread::spawn(move || {
                i + NTHREADS + 2
            }));
        }
        for child in children {
            // Wait for the thread to finish. Returns a result.
            let _ = child.join();
        }
    }
    