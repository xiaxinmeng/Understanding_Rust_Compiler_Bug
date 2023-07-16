rust
fn foo(ref mut vec: A) {
    static move || {
        let vec = vec; // let ref mut vec = vec; once #60501 is merged, AFAICT
    }   
}
