
bad-hashmap-clear.rs:9:8: 9:14 error: illegal borrow unless pure: creating immutable alias to mutable field
bad-hashmap-clear.rs:9         ms.map.insert(1, 2);
                               ^~~~~~
bad-hashmap-clear.rs:9:8: 9:28 note: impure due to access to impure function
bad-hashmap-clear.rs:9         ms.map.insert(1, 2);
                               ^~~~~~~~~~~~~~~~~~~~
