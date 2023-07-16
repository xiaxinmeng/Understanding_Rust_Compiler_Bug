rust 
trait Tr {}

fn foo(_: &dyn Tr) {}

enum E {
    A(Box<dyn Tr>),
}

fn bar(e: &E) {
    match e {
        E::A(i) => {
            foo(i);
        }
    }
}
