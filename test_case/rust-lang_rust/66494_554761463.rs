
error[E0309]: the parameter type `K` may not live long enough
    --> src/libstd/collections/hash/map.rs:2435:9
     |
2427 | impl<'a, K, V, S> Extend<(&'a K, &'a V)> for HashMap<K, V, S>
     |          - help: consider adding an explicit lifetime bound `K: 'a`...
...
2435 |         self.base.extend(iter)
     |         ^^^^^^^^^^^^^^^^^^^^^^
     |
note: ...so that the reference type `&'a K` does not outlive the data it points at
    --> src/libstd/collections/hash/map.rs:2435:9
     |
2435 |         self.base.extend(iter)
     |         ^^^^^^^^^^^^^^^^^^^^^^

error[E0309]: the parameter type `V` may not live long enough
    --> src/libstd/collections/hash/map.rs:2435:9
     |
2427 | impl<'a, K, V, S> Extend<(&'a K, &'a V)> for HashMap<K, V, S>
     |             - help: consider adding an explicit lifetime bound `V: 'a`...
...
2435 |         self.base.extend(iter)
     |         ^^^^^^^^^^^^^^^^^^^^^^
     |
note: ...so that the reference type `&'a V` does not outlive the data it points at
    --> src/libstd/collections/hash/map.rs:2435:9
     |
2435 |         self.base.extend(iter)
     |         ^^^^^^^^^^^^^^^^^^^^^^
