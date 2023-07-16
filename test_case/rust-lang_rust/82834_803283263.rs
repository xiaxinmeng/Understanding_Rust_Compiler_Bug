
thread 'rustc' panicked at 'found unstable fingerprints for evaluate_obligation(62cbf233436d1dfc-eddbe3b43cd9b2ab)', /rustc/f6e313bf13308c2874525016197ab0fa69b53d26/compiler/rustc_query_system/src/query/plumbing.rs:593:5

[...]

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `quote::Tokens: std::marker::Unpin`
#1 [is_unpin_raw] computing whether `quote::Tokens` is `Unpin`
