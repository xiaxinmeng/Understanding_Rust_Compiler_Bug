rust
trait BoxFactory: Fn(&'static str) -> Box<dyn Iterator<Item = &'static str>> {}

impl<T> BoxFactory for T where T: Fn(&'static str) -> Box<dyn Iterator<Item = &'static str>> {}
