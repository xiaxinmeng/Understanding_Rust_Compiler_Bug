
macro_rules! self_calling_method(
    ($m:ident, $name:ident) => (impl SomeType {
        fn $m(&self) { self.$name() }
    })
)

self_calling_method!(a, b)
self_calling_method!(d, e)
