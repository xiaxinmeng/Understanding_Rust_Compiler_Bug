rs
13 | fn bar(a: &str) -> impl Iterator<Item = String> + 'static {
   |           ---- hidden type `impl Iterator<Item = String> + 'static` captures the anonymous lifetime defined here
