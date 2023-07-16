rust
pub trait Data {
    type Elem;
}

pub struct ViewRepr<A>(A);

impl<'a, A> Data for ViewRepr<&'a A> {
    type Elem = A;
}

type ArrayBase<S> = ArrayBaseInner<S, <S as Data>::Elem>;
pub struct ArrayBaseInner<S: Data<Elem = Elem>, Elem> {
    ptr: *mut Elem,
    d: S,
}

fn std1d<'a>(_: ArrayBase<ViewRepr<&'a f64>>) {}

fn map_axis<'a, F>(f: F)
where 
    F: FnMut(ArrayBase<ViewRepr<&'a f64>>)
{}

fn std() {
    map_axis(std1d);
}
