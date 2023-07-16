rust
struct A();
impl A {
    const _1: [(); panic!("1")] = [];
    const _2: () = panic!("2");
}
const _3: () = panic!("3");
