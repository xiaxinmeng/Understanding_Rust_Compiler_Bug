
$ rustc for.rs
for.rs:5:14: 5:16 help: run `rustc --explain E0425` to see a detailed explanation
for.rs:5:5: 5:9 error: failed to resolve. Maybe a missing `extern crate iter`? [E0433]
for.rs:5     for x in xs {}
             ^~~~
for.rs:5:5: 5:9 help: run `rustc --explain E0433` to see a detailed explanation
for.rs:5:5: 5:19 error: unresolved name `iter::IntoIterator::into_iter` [E0425]
for.rs:5     for x in xs {}
             ^~~~~~~~~~~~~~
for.rs:5:5: 5:19 help: run `rustc --explain E0425` to see a detailed explanation
for.rs:5:5: 5:9 error: failed to resolve. Maybe a missing `extern crate iter`? [E0433]
for.rs:5     for x in xs {}
             ^~~~
for.rs:5:5: 5:9 help: run `rustc --explain E0433` to see a detailed explanation
for.rs:5:5: 5:19 error: unresolved name `iter::Iterator::next` [E0425]
for.rs:5     for x in xs {}
             ^~~~~~~~~~~~~~
for.rs:5:5: 5:19 help: run `rustc --explain E0425` to see a detailed explanation
for.rs:5:5: 5:19 error: unresolved enum variant, struct or const `Some` [E0419]
for.rs:5     for x in xs {}
             ^~~~~~~~~~~~~~
for.rs:5:5: 5:19 help: run `rustc --explain E0419` to see a detailed explanation
for.rs:5:5: 5:19 error: unresolved enum variant, struct or const `None` [E0419]
for.rs:5     for x in xs {}
             ^~~~~~~~~~~~~~
