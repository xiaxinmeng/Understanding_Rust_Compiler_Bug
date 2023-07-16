
pub trait Trig<T> {
    pure fn sin(&self) -> T;
}

pub trait Angle<T>: Trig<T> {}

pub pure fn sin<A:Angle<R>, R>(theta: &A) -> R { theta.sin() }

fn foo<R, A:Angle<R>>(theta: A) -> R { sin(&theta) }

impl int: Trig<int> {
    pure fn sin(&self) -> int { 10 }
}

impl int: Angle<int> {
}

fn main() {
    assert foo(0) == 10;
}
