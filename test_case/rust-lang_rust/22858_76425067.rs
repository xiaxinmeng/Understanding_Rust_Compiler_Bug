 rust
let inputs = 0..10;
criterion.bench_prog_with_inputs("py", Command::new("python").arg("under_test.py"), inputs);
