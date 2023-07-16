rust
trait Trait {}

pub fn run(_: &dyn FnOnce(&()) -> Box<dyn Trait + '_>) {}
