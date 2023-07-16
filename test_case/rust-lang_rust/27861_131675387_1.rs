 rust
    let mut v = vec![A{a:1,b:1}, A{a:1,b:2}, A{a:2,b:5}, A{a:3,b:6}];
    let (l, r) = v.split_at_mut(1);
    let a = &mut l[0];
    let b = &mut r[0];
    a.do_stuff(b);
}
