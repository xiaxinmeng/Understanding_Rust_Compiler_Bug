 rust
fn main() {
    let v = vec![(0, vec![0, 1])];
    // this is OK -- the use of `iter` means there's a layer of reference
    // internally whose lifetime you can use for the output reference.
    v.iter().min_by(|v| &v.1); 

    // for some reason, this doesn't work:
    // v.iter().min_by(|v| v); 
    // v.iter().min_by(|v| &*v);     

    // this is not OK, no internal reference:
    // v.into_iter().min_by(|v| &v.1); 
}
