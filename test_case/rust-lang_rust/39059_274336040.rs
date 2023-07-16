
// gate-test-more_struct_aliases
struct S;

trait Tr {
    type A;
}

fn f<T: Tr<A = S>>() {
    let _ = T::A {};
    //~^ ERROR `Self` and associated types in struct expressions and patterns are unstable
}

impl S {
    fn f() {
        let _ = Self {};
        //~^ ERROR `Self` and associated types in struct expressions and patterns are unstable
    }
}

fn main() {}
