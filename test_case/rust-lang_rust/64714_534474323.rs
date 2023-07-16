rust
static COUNT: AtomicUsize = AtomicUsize::new(0);
#[test] fn foo() { for _ in 0..COUNT.load(SeqCst) { println!("!"); } }
#[test] fn bar() { COUNT.write(usize::max_value(), SeqCst); }
