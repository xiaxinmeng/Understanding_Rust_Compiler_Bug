
struct Test<'a, T> {
    t:&'a T
}

fn struct_use() {

    let s = Test { t: &3i32 };
}
