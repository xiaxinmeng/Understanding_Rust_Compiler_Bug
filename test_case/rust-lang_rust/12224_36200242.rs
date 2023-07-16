 rust
// FIXME #11211
type Closure<'a> = 'a |&R|;

struct R<'a, 'b> {
    c: &'a mut Closure<'b>
}

fn innocent_looking_victim() {
    let mut vec = ~[1, 2, 3];
    conspirator(|f| {
        if vec.len() < 100 {
            vec.push(4);
            for _ in vec.iter() {
                (*f.c)(f)
            }
        }
    })
}

fn conspirator(mut f: |&R|) {
    let r = R {c: &mut f};
    f(&r)
}

fn main() {
    innocent_looking_victim()
}
