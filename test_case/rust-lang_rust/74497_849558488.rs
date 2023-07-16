
pub async fn bar() {
    println!("{:?}",foo(|x| baz(x)).await);
}

pub async fn baz(x: &u8) -> bool {
    if *x == 1 {
        false
    } else {
        true
    }
}

pub async fn foo<F, T>(f: F) -> bool
where
    F: Fn(&'static u8) -> T,
    T: Future<Output = bool>,
{
    f(force_static(&32)).await
}

fn force_static<T>(a: &T) -> &'static T {
    unsafe { &*(a as *const T) }
}
