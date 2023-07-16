rust
fn main() {
        let t: std::result::Result<(), ()> = Ok(());
        let f = t
            .into_future()
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(())) // 0.69 secs
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(())) // 1.94 secs
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(())) // 20.68 secs
            .and_then(|_| future::ok(())) // 40.58 secs
            .and_then(|_| future::ok(())) // 80.70 secs
            .and_then(|_| future::ok(())) // 160.59 secs
            .and_then(|_| future::ok(())) // 321.88 secs
            .and_then(|_| future::ok(())); // 642.42 secs 
        f.wait();
}
