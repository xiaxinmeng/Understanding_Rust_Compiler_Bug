
#![feature(box_syntax)]

struct Foo { data: int, next: Option<Box<Foo>> }

fn main() {
    let mut foo = Some(box Foo { data: 0, next: None });
    for i in range(0, 17_000_000) {
        if#![feature(box_syntax)]

struct Foo { data: int, next: Option<Box<Foo>> }

fn main() {
    let mut foo = Some(box Foo { data: 0, next: None });
    for i in range(0, 17_000_000) {
        if i % 1000000 == 0 {
            println!("{}", i);
        }
        foo = Some(box Foo { data: i, next: foo.take() });
    }
}
 i % 1000000 == 0 {
            println!("{}", i);
        }
        foo = Some(box Foo { data: i, next: foo.take() });
    }
}

