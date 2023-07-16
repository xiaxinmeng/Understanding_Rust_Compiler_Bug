rust
use std::future::Future;

pub async fn bar() {
    foo(|x| baz(x)).await;
}

pub async fn baz(x: &u8) -> bool {
    if *x == 1 {
        false
    } else {
        true
    }
}

/// From Nico
///
/// This code is invalid. The closure is given a &u8, which is only valid
/// during the closure execution. But baz(x) captures x and embeds it
/// into a future that is returned -- this future, when awaited, will
/// (or could) attempt to use x, even though the closure itself has
/// returned, and hence x is no longer valid.
///
/// The error message here is obviously not great. We have specialized
/// errors for these kinds of cases in closures normally, I'm not sure
/// why it didn't trigger in this case.
pub async fn foo<F, T>(f: F) -> bool
where
    F: Fn(&u8) -> T,
    T: Future<Output = bool>,
{
    f(&32).await
}

fn main() {}

