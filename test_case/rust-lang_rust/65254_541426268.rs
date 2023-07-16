rust
if let Some(x) = foo && !let Some(y) = bar {
    // we can access x, but not y
} else {
    // what about this scope???
}
