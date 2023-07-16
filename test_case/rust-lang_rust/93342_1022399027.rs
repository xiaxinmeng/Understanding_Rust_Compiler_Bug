rust
#![feature(generic_associated_types)]

use std::marker::PhantomData;

pub trait Scalar: 'static {
    type RefType<'a>: ScalarRef<'a>;
}

pub trait ScalarRef<'a>: 'a {}

impl Scalar for i32 {
    type RefType<'a> = i32;
}

impl Scalar for String {
    type RefType<'a> = &'a str;
}

impl Scalar for bool {
    type RefType<'a> = i32;
}

impl<'a> ScalarRef<'a> for bool {}

impl<'a> ScalarRef<'a> for i32 {}

impl<'a> ScalarRef<'a> for &'a str {}

fn str_contains(a: &str, b: &str) -> bool {
    a.contains(b)
}

pub struct BinaryExpression<A: Scalar, B: Scalar, O: Scalar, F>
where
    // We don't use `Fn` trait here. Instead, we use a bridge trait called `BinaryExprFunc`
    F: BinaryExprFunc<A, B, O>,
{
    f: F,
    _phantom: PhantomData<(A, B, O)>,
}

impl<A: Scalar, B: Scalar, O: Scalar, F> BinaryExpression<A, B, O, F>
where
    F: BinaryExprFunc<A, B, O>,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: PhantomData,
        }
    }
}

pub trait BinaryExprFunc<A: Scalar, B: Scalar, O: Scalar> {
    fn eval(&self, a: A::RefType<'_>, b: B::RefType<'_>) -> O;
}

// We prove that `str_contains` is `Fn(A::RefType<'_>, B::RefType<'_>) -> O` by implementing `BinaryExprFunc`
impl<A: Scalar, B: Scalar, O: Scalar, F> BinaryExprFunc<A, B, O> for F
where
    F: Fn(A::RefType<'_>, B::RefType<'_>) -> O,
{
    fn eval(&self, a: A::RefType<'_>, b: B::RefType<'_>) -> O {
        self(a, b)
    }
}

fn success_case_1() {
    BinaryExpression::<String, String, bool, _>::new(str_contains);
}

fn success_case_2() {
    BinaryExpression::<String, String, bool, _> {
        f: str_contains,
        _phantom: PhantomData,
    };
}

fn main() {
    println!("Hello, world!");
}
