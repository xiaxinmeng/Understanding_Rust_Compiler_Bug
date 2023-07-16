rust
use async_std::task;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

#[async_std::main]
async fn main() {
    let problem = Arc::new(Problem::new());
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

    pub async fn start(self: Arc<Self>) {
        // An arbitrary **SAFE** access to a value held "inside" of self
        let x = self.x.clone();
        
        let arc_self = self.clone();

        // Spawn concurrent task
        task::spawn(async move {
            loop {
                // Some arbitrary other operation that occurs using self (but is not causing a problem)
                x.fetch_sub(1, Ordering::Relaxed);

                arc_self.fn_that_takes_self().await;
            }
        });

        loop {
            // This works fine (as it should)
            self.fn_that_takes_self().await;
        }
    }

    pub async fn fn_that_takes_self(&self) {
        // Use the self variable
        self.x.fetch_add(1, Ordering::Relaxed);
    }
}
