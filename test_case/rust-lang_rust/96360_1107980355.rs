rust
trait Bar {}

trait Foo {}

impl Foo for dyn Bar + Send {}

impl Foo for dyn Bar {}
