 r
/// This is automatically implemented for all types T for which downcasting
/// a type-erased reference of lifetime `'source` to `&'source T` is sound.
/// For example, a simple struct with a lifetime parameter `'a` containing
/// references of lifetime `'a` is `DowncastableFrom<'a>`.
///
/// NB: I don't think this is the same thing as `T: 'a`, but
/// I haven't thought about it too hard.
pub trait DowncastableFrom<'source> {}

pub trait ImprovedAny {
    fn downcast<'a, T>(&'a self) -> &'a T where T: DowncastableFrom<'a>;
}
