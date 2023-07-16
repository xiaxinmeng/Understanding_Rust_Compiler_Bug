 rust
#[cfg(coerce_works3)] // (This one assumes PR 22012 has landed)
pub fn coerce<'a, F>(f: F) -> BoxFn<'a> where F: Fn(), F: 'a { box_!( f ) as BoxFn }
