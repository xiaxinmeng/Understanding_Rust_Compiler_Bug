rs
    foo.bar.f(|| {
        let a = &mut *a;
        let b = &mut *b;
        *if true {a} else {b} = 42;
    });
