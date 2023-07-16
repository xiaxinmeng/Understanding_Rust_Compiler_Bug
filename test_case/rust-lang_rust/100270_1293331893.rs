
pub type Lt<'a, 'b: 'a> = (&'a (), &'b ());

// Compiles
fn check<'a, 'b>(r: &'a &'b ()) {
    let _: Lt<'a, 'b> = (*r, *r);
    let _: Lt<'b, 'a> = (*r, *r);
}
