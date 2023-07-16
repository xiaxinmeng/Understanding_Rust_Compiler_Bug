rust
impl Eq<&str> for &str // becomes:
impl<'a, 'b> Eq<&'a str> for &'b str

trait Foo<'x, T> { ... }

impl Foo<&str> for &str // error: output position `'x` has multiple possibilities

impl Foo<u8> for &str // becomes:
impl<'a> Foo<'a, u8> for &'a str

impl Foo<&str> for u8 // becomes:
impl<'a> Foo<'a, &'a str> for u8
