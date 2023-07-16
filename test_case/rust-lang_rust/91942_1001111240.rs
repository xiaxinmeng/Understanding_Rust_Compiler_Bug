rust
fn debug(
    &self,
    s: &mut State<'db, '_>,
    callable: &mut impl Fn(&mut State<'db, '_>),  // <-----
)
