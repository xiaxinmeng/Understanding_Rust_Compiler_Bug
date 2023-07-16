
async fn foo() {
    let _ = await bar();
}
async fn foo() {
    let _ = await? bar();
}
async fn foo() {
    let _ = await bar()?;
}
async fn foo() {
    let _ = (await bar())?;
}
async fn foo() {
    let _ = bar().await();
}
async fn foo() {
    let _ = bar().await()?;
}
async fn foo() {
    let _ = bar().await;
}
async fn foo() {
    let _ = bar().await?;
}
fn foo() {
    let _ = await bar();
}
fn foo() {
    let _ = await? bar();
}
fn foo() {
    let _ = await bar()?;
}
fn foo() {
    let _ = (await bar())?;
}
fn foo() {
    let _ = bar().await();
}
fn foo() {
    let _ = bar().await()?;
}
fn foo() {
    let _ = bar().await;
}
fn foo() {
    let _ = bar().await?;
}
