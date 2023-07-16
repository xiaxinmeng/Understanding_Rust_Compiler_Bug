rust
#![feature(nll)]

enum SimpleFoo {
    Ref(u8)
}

fn bar(mut foo: SimpleFoo) {
    match foo {
        SimpleFoo::Ref(ref mut inner) | SimpleFoo::Ref(ref mut inner) => {
            println!("Inner: {:?}", inner);
        },
    }
}

fn main() {
    bar(SimpleFoo::Ref(5));
}
