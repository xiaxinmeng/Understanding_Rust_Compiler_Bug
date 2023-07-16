
trait Context {
    type Container: ?Sized;
}

struct Wrapper< C: Context > where < C as Context >::Container: 'static {
    container: &'static C::Container
}

struct Foobar;

impl Context for Foobar {
    type Container = ();
}

fn foobar() -> Wrapper< Foobar > {
    unimplemented!();
}

fn main() {}
