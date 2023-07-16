rust
#![feature(const_generics)]

struct IsizeVal<const X: isize>;
impl<const X: isize> IsizeVal<X> {
    const VAL: isize = X;
}

enum Bar</*FOO*/> {
    X = {
        const FOO: isize = 3;
        IsizeVal::<FOO>::VAL
    }
}
