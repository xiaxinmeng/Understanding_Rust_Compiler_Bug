 rust
fn apply<A, B, C, F, G>(mut f: F, a: A) 
-> impl FnMut(&B) -> C // must still be `for<'r> impl FnMut(&'r B) -> C`, because that’s what filter requires
         where F: FnMut(B) -> G, // must not be `for<'r> FnMut(&'r B) -> G`, because regular functions do not implement it
               G: FnMut(A) -> C,
               B: Copy, // for dereferencing
               A: Clone {

    move |b| f(*b)(a.clone()) // this must do any bridging necessary to satisfy the requirements between filter and regular functions
}
