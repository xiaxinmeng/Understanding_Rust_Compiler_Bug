
enum E {
    Foo(EFoo),
    Bar(EBar),
}

struct EFoo(u32);
struct EBar(f32);

fn foo(e: &mut E) -> &mut E {
    match e {
        E::Foo(a1) => {
            println!("{}", &a1.0);
            a1
        },
        x => x
    }
}
