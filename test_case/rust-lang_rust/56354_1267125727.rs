rust
match 0usize {
    0..=9 => { /* some code that only runs on small numbers */ },
    10.. => { /* some code that handles everything else */ },
}
