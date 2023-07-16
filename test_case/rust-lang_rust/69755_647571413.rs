rust
fn y<A, O>(f: impl Fn(&dyn Fn(A) -> O, A) -> O) -> impl Fn(A) -> O {
    struct X<'a, A, O>(&'a dyn Fn(X<A, O>, A) -> O);

    impl<'a, A, O> Clone for X<'a, A, O> {
        fn clone(&self) -> Self {
            Self(self.0)
        }
    }

    impl<'a, A, O> Copy for X<'a, A, O> {}

    impl<'a, A, O> X<'a, A, O> {
        fn call(&self, x: Self, a: A) -> O {
            (self.0)(x, a)
        }
    }

    move |a| (|x: X<A, O>, a| x.call(x, a))(X(&|x, a| f(&|a| x.call(x, a), a)), a)
}

#[inline(never)]
fn fact(n: usize) -> usize {
    y(|f, n| match n {
        0 | 1 => 1,
        n => n * f(n - 1),
    })(n)
}

#[inline(never)]
fn fib(n: usize) -> usize {
    y(|f, n| match n {
        0 => 0,
        1 => 1,
        n => f(n - 1) + f(n - 2),
    })(n)
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();
    dbg!(fib(n));
    dbg!(fact(n));
}
