rust
use std::time;

futures_timer::wait_for(30 * time::SECONDS).await;
for instant in futures_timer::repeat(5 * time::MINUTES).await {
    println!("the time is {}", instant);
}
