rust
use async_std::task;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

#[async_std::main]
async fn main() {
    let problem = Problem::new();
    problem.start().await;
}

struct Problem {
    x: Arc<AtomicU32>,
}

impl Problem {
    pub fn new() -> Problem {
        Problem {
            x: Arc::new(AtomicU32::new(0)),
        }
    }

    pub async fn start(&self) {
        // An arbitrary **SAFE** access to a value held "inside" of self
        let x = self.x.clone();

        // Spawn concurrent task
        task::spawn(async move { // <== This whole block is shown as the error, even though the problem lies within one line
            loop {
                // Some arbitrary other operation that occurs using self (but is not causing a problem)
                x.fetch_sub(1, Ordering::Relaxed);

                // A call on self that is causing the problem, but not explained in the error
                self.fn_that_takes_self().await;
            }
        });

        loop {
            // This works fine
            self.fn_that_takes_self().await;
        }
    }

    pub async fn fn_that_takes_self(&self) {
        // Use the self variable
        self.x.fetch_add(1, Ordering::Relaxed);
    }
}
