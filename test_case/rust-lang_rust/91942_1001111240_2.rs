rust
fn debug<'a>(
    &self,
    s: &mut State<'db, '_>,
    callable: &mut impl Fn(&mut State<'db, 'a>),
)
