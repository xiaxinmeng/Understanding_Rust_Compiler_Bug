
type S = uint;
type T = uint;

fn main() {
    obj a() {
        fn foo() -> S { ret 2u; }
    }
    let my_a = a();

    let my_b = obj() {
        fn foo() -> T { ret 3u; } // Type checker reports an error
        with my_a
    };

    assert my_b.foo() == 3u;
}
