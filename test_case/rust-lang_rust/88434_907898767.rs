
// build-pass
// Regression test for issue 88434

const _CONST: &[u8] = &f(&[], |_| {});

const fn f<F>(_: &[u8], _: F) -> &[u8]
where
    F: FnMut(&'static u8),
{
    panic!()
}

fn main() {}
