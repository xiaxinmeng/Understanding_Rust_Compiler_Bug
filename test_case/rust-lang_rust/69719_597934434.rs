rust
trait Tr1 {}

trait Tr2: Tr1 {}

impl<T: Tr2 + Sized> Tr1 for T {}

//impl Tr1 for [u8] {}

impl Tr2 for () {}

impl Tr2 for [u8] {}
