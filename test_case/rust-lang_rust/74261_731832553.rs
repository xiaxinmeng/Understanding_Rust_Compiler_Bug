rust
pub trait Ref<'a> {
    type Type: Clone;
}
impl<'a, T: 'static + Clone> Ref<'a> for T {
    type Type = T;
}

pub type EvalFn<I> = dyn FnMut(&<I as Ref<'_>>::Type);

pub trait Evaluator<T: for<'a> Ref<'a>> {
    fn make_evaluator(&self) -> Box<EvalFn<T>>;
}

pub struct A;

impl Evaluator<i32> for A {
    fn make_evaluator(&self) -> Box<EvalFn<i32>> {
        Box::new(|_x| {})
    }
}

fn main() {
    let mut x: Vec<Box<EvalFn<i32>>> = vec![];
    x.push(A.make_evaluator());
}
