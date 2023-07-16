
async fn foo() {
    let baz = vec![(); 10];
    let mut v = vec![];
    for n in baz.iter() {
        v.push(async move {
            bar(n).await;
        })
    }
    for x in v {
        x.await;
    }
}

async fn bar(n: &()) {}
