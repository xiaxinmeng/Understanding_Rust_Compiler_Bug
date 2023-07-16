rust
trait Trait {}

struct Struct;

impl Trait for Struct {}

fn never_return() -> ! {
    loop {}
}

fn example() -> impl Trait {
    never_return()
}
