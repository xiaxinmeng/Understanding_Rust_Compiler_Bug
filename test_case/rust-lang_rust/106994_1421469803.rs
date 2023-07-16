rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

trait Tr {
    const C: usize = 0;
}

struct Str {}
impl Tr for Str {}

// ICE, but ONLY if this generic type is used (in a function signature, an expression...).
struct ICE<T: Tr = Str, const C: usize = {T::C}>
where [(); C]:,
{
    _t: core::marker::PhantomData<T>,
    _arr: [(); C],
}

// Using the above type in a function signature, or in an expression causes an ICE.

fn _type_in_fn_sig_return_causes_ice() -> ICE {
    loop{}
}
// and/or:

fn _type_in_fn_sig_param_causes_ice(_: &ICE) {
    loop{}
}
// and/or:

fn _instantiate() {
    ICE::<Str> {
        _t: core::marker::PhantomData {},
        _arr: []
    };
}
