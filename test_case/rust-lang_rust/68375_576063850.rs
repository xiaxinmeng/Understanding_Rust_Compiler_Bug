rust
pub trait Data {
    type Elem;
}

pub struct ViewRepr<A>(A);

impl<'a, A> Data for ViewRepr<&'a A> {
    type Elem = A;
}


type ArrayBase<D> = ArrayBaseInner<D, <D as Data>::Elem>;
pub struct ArrayBaseInner<D: Data<Elem = Elem>, Elem> {
    ptr: *mut Elem,
    d: D,
}

pub fn cast_array_view<'shorter, 'longer: 'shorter>(
    input: ArrayBase<ViewRepr<&'longer f32>>
) -> ArrayBase<ViewRepr<&'shorter f32>> {
    input
}
