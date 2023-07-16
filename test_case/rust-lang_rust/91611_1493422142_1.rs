
error[E0728]: `await` is only allowed inside `async` functions and blocks
  --> src\generation\iterator.rs:25:54
   |
24 | /     fn iterate(&self) -> impl Stream<Item = usize> + '_ {
25 | |         tokio::time::sleep(Duration::from_millis(10)).await;
   | |                                                      ^^^^^^ only allowed inside `async` functions and blocks
26 | |
27 | |         stream! {
...  |
34 | |         }
35 | |     }
   | |_____- this is not `async`
