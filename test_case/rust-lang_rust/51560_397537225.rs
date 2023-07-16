rust
#[must_use]
struct Foo {}

#[must_use]
trait MustUseTrait {}
impl MustUseTrait for Foo {}

trait CanUseTrait {}
impl CanUseTrait for Foo {}

fn get_foo() -> Foo { Foo {} }
fn get_must_use_trait() -> impl MustUseTrait { Foo {} }
fn get_can_use_trait() -> impl CanUseTrait { Foo {} }

fn main() {
    get_foo(); // Warning (because of must_use on Foo and MustUseTrait)
    get_must_use_trait(); // Warning (because of must_use on MustUseTrait)
    get_can_use_trait(); // No warning (must_use on Foo does not matter because the type is anonymous)
}

