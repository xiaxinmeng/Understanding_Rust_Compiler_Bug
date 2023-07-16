
trait Foo {}

impl<'self> Foo for &'self str {}

...
let _ = & &"foo" as &Foo;
