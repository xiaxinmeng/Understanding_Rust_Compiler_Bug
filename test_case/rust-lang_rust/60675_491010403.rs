rust
macro_rules! r#await {
    ($e:expr) => { 
        $e.await
    }
}
