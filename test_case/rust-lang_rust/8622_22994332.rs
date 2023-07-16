
struct R<'self> {
    c: &'self fn(&R)
}

fn innocent_looking_victim() {
    let mut vec = ~[1, 2, 3];
    conspirator(|f| {
        if vec.len() < 100 {
            vec.push(4);
            for vec.each |i| {
                f.c(&f)
            }
        }
    })
}

// only this part is different
fn conspirator(f: &mut fn(&R)) {
    conspirator_impl(f);
}

fn conspirator_impl(f: &fn(&R)) {
    let r = R {c: f};
    f(&r)
}
