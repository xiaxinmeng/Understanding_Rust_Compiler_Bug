 rust
for x in it.by_ref() {  // advance the iterator
    // ..
}

match it.next() {  // reuse the mutated iterator
    // ..
}
