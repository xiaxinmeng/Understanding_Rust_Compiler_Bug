
$ rustc --version
rustc 1.31.1 (b6c32da9b 2018-12-18)

$ cat rust-issue-49148.rs 
fn a(n: int) {}

fn main() {
  let b = vec![];
  let c = 0;
  a(b[c;
}

$ rustc rust-issue-49148.rs 
$ rustc rust-issue-49148.rs 
error: incorrect close delimiter: `}`
 --> rust-issue-49148.rs:7:1
  |
3 | fn main() {
  |           - close delimiter possibly meant for this
...
6 |   a(b[c;
  |      - un-closed delimiter
7 | }
  | ^ incorrect close delimiter

error: expected one of `!`, `.`, `::`, `?`, `]`, `{`, or an operator, found `;`
 --> rust-issue-49148.rs:6:8
  |
6 |   a(b[c;
  |        ^ expected one of 7 possible tokens here

error[E0412]: cannot find type `int` in this scope
 --> rust-issue-49148.rs:1:9
  |
1 | fn a(n: int) {}
  |         ^^^ not found in this scope

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0412`.
