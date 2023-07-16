
struct A<T> {
    a: T
}

fn f<T>(v: A<T>) {
    let ~A{a: ref a} = v;
}
