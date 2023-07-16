rust
#[derive(Clone)]
struct A;

trait CloneableFn: Fn(&A) + CloneBox {}

trait CloneBox {
    fn clone_box(&self) -> Box<dyn CloneableFn>;
}

impl<T> CloneableFn for T where T: Fn(&A) + CloneBox {}

impl<T> CloneBox for T where T: Fn(&A) + Clone + 'static {
    fn clone_box(&self) -> Box<dyn CloneableFn> {
        Box::new(self.clone()) as Box<dyn CloneableFn>
    }
}


fn take_cfn(cfn: Box<dyn CloneableFn>, a: A) {
    cfn(&a);
}

fn main() {
    let a = A;
    let closure = move |_| { // solve with `move |_: &A|`
        let _b = a.clone();
    };
    take_cfn(Box::new(closure) as Box<dyn CloneableFn>, A);
}
