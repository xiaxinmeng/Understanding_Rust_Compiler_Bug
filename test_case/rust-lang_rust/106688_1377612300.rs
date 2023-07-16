
struct A;
struct B<'b>(&'b mut A);

async fn m(_b: &mut B<'_>) {}

async fn n2(b: &mut B<'_>) {
    m(b).await
}

trait TxHelper<'a, I, O> {
    type Output: 'a + std::future::Future<Output = O>;

    fn call(self, arg: &'a mut I) -> Self::Output;
}

impl<'a, I, O, F, N> TxHelper<'a, I, O> for N
where
    I: 'a,
    F: 'a + std::future::Future<Output = O>,
    N: FnOnce(&'a mut I) -> F {
    type Output = F;

    fn call(self, arg: &'a mut I) -> F {
        self(arg)
    }
}

async fn tx<
    N: for<'b> TxHelper<'b, B<'b>, ()>,
>(a: &mut A, n: N) {
    let mut b = B(a);
    n.call(&mut b).await;
    drop(b);
}

#[tokio::main]
pub async fn main() {
    tx(
        &mut A, 
        n2,
    ).await;
}
