rust
trait Trait {}

trait Alias = Trait;

impl Alias for () {}
