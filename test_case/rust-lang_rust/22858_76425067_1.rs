 rust
// one call for each value of `$N`
criterion.bench_prog("py/$N", Command::new("python").arg("under_test.py").arg("$N"));
...
// and the user must not forget to summarize the results at the end
criterion.summarize("py");
