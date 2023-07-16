rust
trait A {
    type Projection;
}

impl A for () {
    type Projection = bool;
    // using () instead of bool here does compile though
}

struct Next<T: A>(T::Projection);

fn e(item: Next<()>) {
    match item {
        Next(true) => {}
        Next(false) => {}
    }
}

fn main() {}
