
$ rustc main.rs
error: expected one of `(`, `)`, `,`, `::`, or `=`, found `==`
 --> main.rs:3:30
  |
3 |        #[cfg(any(target_arch == "arm"))]
  |                              ^^ expected one of `(`, `)`, `,`, `::`, or `=`

Aborted (core dumped)
