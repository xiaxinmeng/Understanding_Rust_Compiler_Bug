
pure fn zip<T, U>(+v: ~[const T], +u: ~[const U]) -> ~[(T, U)] {
    let i = len(v);
    assert i == len(u);
    let w = ~[];
    while i > 0 {
        push(w, (pop(v),pop(u)));
    }
    reverse(w);
    w
}
