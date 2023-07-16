 rust
// original code
for x in it {  // a *copy* of the iterator is used here
    // ..
}

match it.next() {  // the original iterator is used here
    // ..
}
