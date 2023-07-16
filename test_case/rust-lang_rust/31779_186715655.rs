 rust
mod foo {
    trait T { // This should be private to `foo`
        type Assoc;
    }

    impl T for () {
        type Assoc = super::S;
    }
}

struct S;
impl S {
    fn f() {}
}

fn main() {
    <() as foo::T>::Assoc::f(); // but it can be used here
}
