plain
   Compiling toml v0.5.9
error[E0382]: borrow of moved value: `run`
   --> dist.rs:100:55
    |
99  |     fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
    |                   --- move occurs because `run` has type `ShouldRun<'_>`, which does not implement the `Copy` trait
100 |         run.alias("rust-json-docs").default_condition(run.builder.config.docs)
    |             |
    |             |
    |             `run` moved due to this method call
    |
note: this function takes ownership of the receiver `self`, which moves `run`
    |
    |
438 |     pub fn alias(mut self, alias: &str) -> Self {
    |                      ^^^^
    = note: borrow occurs due to deref coercion to `Build`
note: deref defined here
    |
47  |     type Target = Build;
    |     ^^^^^^^^^^^

