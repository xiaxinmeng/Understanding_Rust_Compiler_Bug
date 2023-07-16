rust
// add this line:
use futures::stream::{StreamExt};
use lazy_static::lazy_static;
use rand::distributions::{Distribution, Uniform};
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::io::Error;
use std::time::Duration;
use tokio::time::{sleep, Instant};

lazy_static! {
    static ref START_TIME: Instant = Instant::now();
}

// add this module:
pub mod p2p {
    //omit this line
    //use futures::stream::{StreamExt};

    pub async fn handle_signals(signals: signal_hook_tokio::Signals) {
        let mut signals = signals.fuse();
        while let Some(signal) = signals.next().await {
            match signal {
                signal_hook::consts::SIGTERM | signal_hook::consts::SIGINT | signal_hook::consts::SIGQUIT => {
                    // Lets get out of here...
                    println!("Trapped signal to quit. Exiting");
                    std::process::exit(1);
                }
                _ => unreachable!(),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let signals = signal_hook_tokio::Signals::new(&[SIGHUP, SIGTERM, SIGINT, SIGQUIT])?;
    let handle = signals.handle();
    // remember the module
    let signals_task = tokio::spawn(p2p::handle_signals(signals));

    let page = get_page(42).await;
    println!("Page #42: {:?}", page);

    // Terminate the signal stream.
    handle.close();
    signals_task.await?;
    Ok(())
}

async fn get_page(i: usize) -> Vec<usize> {
    let millis = Uniform::from(5_000..6_000).sample(&mut rand::thread_rng());
    println!(
        "[{}] # get_page({}) will complete in {} ms on {:?}",
        START_TIME.elapsed().as_millis(),
        i,
        millis,
        std::thread::current().id()
    );

    sleep(Duration::from_millis(millis)).await;
    println!(
        "[{}] # get_page({}) completed",
        START_TIME.elapsed().as_millis(),
        i
    );

    (10 * i..10 * (i + 1)).collect()
}
