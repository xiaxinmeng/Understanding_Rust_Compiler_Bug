rust
#[repr(C)]
#[derive(Debug)]
struct PairFoo {
    fst: Foo,
    snd: Foo
}

#[derive(Debug)]
struct Foo(u64);

fn reinterstruct(ptr_pair: Box<PairFoo>) -> Box<[Foo]> {
    unsafe {
        Vec::from_raw_parts(&mut Box::leak(ptr_pair).fst as *mut Foo, 2, 2)
            .into_boxed_slice()
    }
}

fn main() {
    let pair_foo = Box::new(PairFoo{ fst: Foo(42), snd: Foo(1337) });
    println!("pair_foo = {:?}", pair_foo);
    for (n, foo) in reinterstruct(pair_foo).into_iter().enumerate() {
        println!("foo #{} = {:?}", n, foo);
    }
}
