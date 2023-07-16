console
$ rustc a.rs && ./a
Location { file: "a.rs", line: 9, col: 5 }
$ rustc a.rs -O && ./a
Location { file: "a.rs", line: 4, col: 13 }
