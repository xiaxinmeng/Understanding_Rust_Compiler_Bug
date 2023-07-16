rust
fn spawn_a_task(&self, mut somesignal: mpsc::Receiver<K>) {
    tokio::spawn(async move {
        while let Some(foo) = &somesignal.recv().await { /* omitted */ }
    }
}
