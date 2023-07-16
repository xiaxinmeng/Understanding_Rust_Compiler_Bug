
trait Foo {}

trait Bar {}

impl Foo for Bar {}
//~^ should suggest `impl<T: Bar> Foo for T {}`

fn main() {}
