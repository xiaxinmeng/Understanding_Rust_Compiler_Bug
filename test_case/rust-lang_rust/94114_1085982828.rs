rust
let mut future: impl Future;
'a: {
    Pin::new(&'a mut future).poll();
}
// 'a is over, future "isn't" pinned
// caveat emptor: yes it is, per current pin documentation
forget(future);
// reuse future's backing memory
// if the async runtime stored a pointer to the future, 💣
