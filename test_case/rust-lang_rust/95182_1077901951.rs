
async fn do_say_hi<'a, F, R>(func: F)
where
F: Fn(&'a str) -> R,
R: Future<Output = ()> + 'a {
  (func)("world").await;
}
