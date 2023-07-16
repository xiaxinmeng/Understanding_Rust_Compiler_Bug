rust
trait Trait {}

type TAIT = impl Trait;

struct Bar;
impl Trait for Bar {}

fn foo() -> TAIT {
    Bar
}

// ok
async fn bar() -> (TAIT, Bar) {
   (foo(), Bar)
}

// err: concrete type differs
// async fn bar() -> (TAIT, impl Trait) {
//     (foo(), Bar)
// }

// err: concrete type differs
// async fn bar() -> (TAIT, Box<dyn Trait>) {
//     (foo(), Box::new(Bar))
// }
