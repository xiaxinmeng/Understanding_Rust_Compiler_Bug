
struct A;
mod foo {
    use super::*;
    mod bar {
        use super::*;
        fn a() -> A {
            A
        }
    }
}
