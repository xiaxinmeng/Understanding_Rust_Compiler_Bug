rust
fn f3(mut s: Pin<&mut S>) {
    let sr = &mut *s; // explicitly invoke Pin's DerefMut first. sr exclusively borrows the entire s
    let r = &mut sr.a; // r only borrows sr.a
    println!("{}", sr.b); // OK, because only sr.a was borrowed. We can still access sr.b
    drop(r);
}
