
fn main() {

    obj a() {
        fn foo() -> int {
            ret 2;
        }
    }

    auto my_a = a();

    auto my_b = obj() {
        fn foo() -> int {
            ret 3;
        }
        with my_a
    };

    assert (my_b.foo() == 3);
}
