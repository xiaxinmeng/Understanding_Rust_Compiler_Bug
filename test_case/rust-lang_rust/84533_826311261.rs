rust
use std::marker::PhantomData;

fn fun<'b, 'a>(_: &'b (), _: &'a ()) -> PhantomData<&'b &'a ()> {
    PhantomData
}

fn use_fun() {
    blah(fun);
    blubb(fun);
}

fn use_closure() {
    blah(|_, _| PhantomData);
    blubb(|_, _| PhantomData);
}

fn blah(f: for<'b, 'a> fn(&'b (), &'a ()) -> PhantomData<&'b &'a ()>) {
    blubb(f);
}

fn blubb(f: impl for<'b, 'a> Fn(&'b (), &'a ()) -> PhantomData<&'b &'a ()>) {}
