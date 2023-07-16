rust
struct Newtype<'a>(Ptr<&'a ()>);

fn covariant_newtype<'a>(p: Newtype<'static>) -> Newtype<'a> { p }
