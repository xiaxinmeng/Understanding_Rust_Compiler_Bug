
error[E0507]: cannot move out of captured variable in an `FnMut` closure
  --> src/deps.rs:51:29
   |
42 |           root_node: Arc<RootNode<'a, QueryT, MutationT, S>>,
   |           --------- captured outer variable
...
51 |               future::poll_fn(move || {
   |  _____________________________^
52 | |                 let _res = request.execute(&root_node);
53 | |                 Ok(Async::Ready(()))
54 | |             })
   | |_____________^ cannot move out of captured variable in an `FnMut` closure

error: aborting due to previous error
