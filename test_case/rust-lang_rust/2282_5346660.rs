
type T/& = {
    mut f: &uint
};

enum E/& = T;

fn foo(x: &E, v: &uint) {
    x.f = v;
}

fn bar(x: &E) {
    let v = 3u;
    foo(x, &v);
}
