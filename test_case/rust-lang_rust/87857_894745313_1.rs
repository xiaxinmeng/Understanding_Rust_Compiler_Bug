rust
   |19 |         let mut signals = signals.fuse();
   |                                   ^^^^ method cannot be called on `signal_hook_tokio::SignalsInfo` due to unsatisfied trait bounds   |
  ...
   |
92 | pub struct SignalsInfo<E: Exfiltrator = SignalOnly>(OwningSignalIterator<UnixStream, E>);
   | ----------------------------------------------------------------------------------------- doesn't satisfy `signal_hook_tokio::SignalsInfo: Iterator`
   |
   = note: the following trait bounds were not satisfied:
           `signal_hook_tokio::SignalsInfo: Iterator`
           which is required by `&mut signal_hook_tokio::SignalsInfo: Iterator`
   = help: items from traits can only be used if the trait is in scope
   = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
           `use futures::StreamExt;`
