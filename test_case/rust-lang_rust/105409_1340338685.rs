diff
use std::future::Future;

fn hrc<
    R,
    F: for<'a> AsyncClosure<'a, (), R>
        + for<'a> Fn(&'a ()) -> <F as AsyncClosure<'a, (), R>>::Fut,
>(
    f: F,
) -> F {
    f
}

fn main() {
-    hrc(|x| async { });
+    hrc(|x: &_| async { });
}

trait AsyncClosure<'a, I, R>: Fn(&'a I) -> Self::Fut
where
    I: 'a,
{
    type Fut: Future<Output = R> + Send + 'a;
}

impl<'a, I, R, Fut, F> AsyncClosure<'a, I, R> for F
where
    I: 'a,
    F: Fn(&'a I) -> Fut,
    Fut: Future<Output = R> + Send + 'a,
{
    type Fut = Fut;
}
