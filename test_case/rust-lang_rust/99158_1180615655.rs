rs
// type Inv<'a> = std::cell::Cell<&'a ()>;
type Inv<'a> = &'a ();

fn extend<'a, 'b>(x: &'a str) -> &'b str {
    let f: fn(_, &'a str) -> &'b str = extend_with_proof::<'a, 'b>;
    call_with_fake_proof::<'a, 'b>(f, x)
}

fn extend_with_proof<'a: 'a, 'b: 'b>(
    _proof: std::marker::PhantomData<&'b Inv<'a>>,
    x: &'a str,
) -> &'b str {
    x
}

fn call_with_fake_proof<'a: 'a, 'b: 'b, T: Default>(
    extend_with_proof_ptr: fn(_proof: T, &'a str) -> &'b str,
    s: &'a str,
) -> &'b str {
    extend_with_proof_ptr(Default::default(), s)
}
