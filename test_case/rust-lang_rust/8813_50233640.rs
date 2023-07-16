 Rust
trait TraitA {}

struct A;

impl TraitA for A {}

fn foo1<T: TraitA>(something: &T) {
    object_bar(something);
    object_bar(something);
}

fn foo2<T: TraitA>(something: &mut T) {
    generic_bar(something);
    generic_bar(something);
}

fn foo3<T: TraitA>(something: &mut T) {
    mut_object_bar(something);
    mut_object_bar(something);
}

fn foo4<T: TraitA>(something: &mut T) {
    object_bar(something);
    object_bar(something);
}

fn object_bar(_something: &TraitA) {}
fn mut_object_bar(_something: &mut TraitA) {}
fn generic_bar<T: TraitA>(_something: &mut T) {}

fn main() {
    let mut a = A;

    foo1(&a);
    foo2(&mut a);
    foo3(&mut a);
    foo4(&mut a);
}
