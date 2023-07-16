 rust
    let f : Box<Foo<[int, ..3]>> = box Foo { y: [5i, 6, 7] };
    let g : Box<Foo<[int]>> = f;
    println!("&g.y: {:?}", &g.y);
