rust
fn main() {
    use std::thread;
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    let n = 10;
    let foos: Vec<_> = (0..n)
        .into_iter()
        .map(|i| thread::spawn(move || -> bool { rx.recv().unwrap() }))
        .collect();
}
