 rust
trait X {}
trait Y {}
trait Z {}
impl <T : Y> X for T {}
impl <T : Z> X for T {}
