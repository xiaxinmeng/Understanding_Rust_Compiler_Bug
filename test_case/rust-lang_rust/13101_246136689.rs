
#[derive(PartialEq)]
struct NotEq;

#[derive(PartialEq, Eq)]
struct S {
    a: NotEq,
}
