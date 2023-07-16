rs
    let list = vec![];

    foo(&mut a, |b, []| Box::pin(baz(b, &list))).await;
}

async fn foo<'a, 'env, F>(a: &'a mut A, f: F)
where
     F: for<'b> FnOnce(&'b mut B, [&'b &'env (); 0]) -> BoxFuture<'b, ()>,
{

    f(a.b_mut(), []).await;
}
