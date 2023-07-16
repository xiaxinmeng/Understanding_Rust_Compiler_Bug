 rust
fn f<'a>(x: &'a Option<String>) -> &'a str {
     match *x {
         Some(ref s) => s.as_slice(),
         None => "<none>"
     }
}
