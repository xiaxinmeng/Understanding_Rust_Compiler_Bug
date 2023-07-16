console
> touch src/lib.rs && time cargo test --all-features --test docs
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
cargo test --all-features --test docs  0.46s user 0.19s system 103% cpu 0.635 total

> touch src/lib.rs && time cargo test --all-features --doc
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
cargo test --all-features --doc  13.20s user 2.85s system 277% cpu 5.777 total
