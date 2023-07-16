 rust
    use std::mem;
    let mut v = vec![A{a:1,b:1}, A{a:1,b:2}, A{a:2,b:5}, A{a:3,b:6}];
    let i = 0;
    let j = 1;
    let mut a = mem::replace(&mut v[i], A{a:0,b:0});
    let mut b = mem::replace(&mut v[j], A{a:0,b:0});

    a.do_stuff(&mut b);

    mem::replace(&mut v[i], a);
    mem::replace(&mut v[j], b);
