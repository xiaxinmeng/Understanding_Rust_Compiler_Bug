
error[E0621]: explicit lifetime required in the type of `string`
  --> src\lib.rs:43:22
   |
36 | pub fn count_words_parallel(string: &str, threads: usize)
   |                                     ---- help: add explicit lifetime `'static` to the type of `string`: `&'static str`
...
43 |         handles.push(thread::spawn(|| {
   |                      ^^^^^^^^^^^^^ lifetime `'static` required
