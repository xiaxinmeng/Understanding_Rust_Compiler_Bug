rust
while !flag.load(Ordering::Relaxed) {
   thread::park();
}
