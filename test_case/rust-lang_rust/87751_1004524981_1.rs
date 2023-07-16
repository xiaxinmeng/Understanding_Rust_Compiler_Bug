text
help: elided lifetimes in `where`-clause restrictions require an explicit `for<>` bound
  |
1 | fn bar<R>(r: &R) where for<'a> &'a R: std::io::Read {
  |                        ^^^^^^^  ^^
