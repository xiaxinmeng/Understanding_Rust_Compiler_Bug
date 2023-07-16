rust
trait Test {}

impl Test for fn(&'static u8) { }
impl Test for for<'a> fn(&'a u8) { }
