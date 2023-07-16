rust
trait X {}
impl X for fn(fn(&())) {}
impl X for fn(fn(&'static ())) {}
