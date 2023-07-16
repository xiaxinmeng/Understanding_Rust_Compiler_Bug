
struct S<T: Const> {
    s: T,
    mut cant_nest: ()
}

fn main() {
    let a1  = ~S{ s: true, cant_nest: () };
    let _a2 = ~S{ s: move a1, cant_nest: () };
}
