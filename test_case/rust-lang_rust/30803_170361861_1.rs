
foo.rs:13:5: 13:9 error: failed to resolve. Maybe a missing `extern crate iter`? [E0433]
foo.rs:13     for _ in () {
              ^~~~
foo.rs:13:5: 13:9 help: run `rustc --explain E0433` to see a detailed explanation
foo.rs:13:5: 14:6 error: unresolved name `iter::IntoIterator::into_iter` [E0425]
foo.rs:13     for _ in () {
foo.rs:14     }
foo.rs:13:5: 14:6 help: run `rustc --explain E0425` to see a detailed explanation
foo.rs:13:5: 13:9 error: failed to resolve. Maybe a missing `extern crate iter`? [E0433]
foo.rs:13     for _ in () {
              ^~~~
foo.rs:13:5: 13:9 help: run `rustc --explain E0433` to see a detailed explanation
foo.rs:13:5: 14:6 error: unresolved name `iter::Iterator::next` [E0425]
foo.rs:13     for _ in () {
foo.rs:14     }
foo.rs:13:5: 14:6 help: run `rustc --explain E0425` to see a detailed explanation
foo.rs:13:5: 14:6 error: unresolved enum variant, struct or const `Some` [E0419]
foo.rs:13     for _ in () {
foo.rs:14     }
foo.rs:13:5: 14:6 help: run `rustc --explain E0419` to see a detailed explanation
foo.rs:13:5: 14:6 error: unresolved enum variant, struct or const `None` [E0419]
foo.rs:13     for _ in () {
foo.rs:14     }
