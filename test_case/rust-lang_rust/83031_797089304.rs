rust
pub async fn my_middleware<N, Fut>(route: N)
where
    N: FnOnce() -> Fut,
    Fut: Future<Output = ()>,
{
    (| | async { (| | async { (| | async { (| | async { (| | async { (| | async { (| | async { (| | async { (| | async { (| | async { (| | async { (| | async move { route () . await }) () . await }) () . await }) () . await }) () . await }) () . await }) () . await }) () . await }) () . await }) () . await }) () . await }) () . await }) () . await
}
