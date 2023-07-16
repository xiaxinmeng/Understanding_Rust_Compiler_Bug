rust
fn foo<'a>(&self, x: &'a i32) -> &i32 {
    //               -------     ---- these references must have the same lifetime
    x
//  ^ data from `x` is returned here
}

fn foo<'a>(&self, x: &'a i32) -> &i32 {
    //               -------     ---- these references must have the same lifetime
    if true { &self.field } else { x }
    //                             ^ data from `x` is returned here
}
