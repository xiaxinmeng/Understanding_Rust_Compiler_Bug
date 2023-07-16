
oo.rs:9:23: 9:35 error: no method named `contains_key` found for type `std::collections::hash::map::HashMap<T, std::collections::hash::set::HashSet<T>>` in the current scope
foo.rs:9         if ! self.map.contains_key(node) {
                               ^~~~~~~~~~~~
foo.rs:9:23: 9:35 note: the method `contains_key` exists but the following trait bounds were not satisfied: `T : core::cmp::Eq`, `T : core::hash::Hash`
foo.rs:10:22: 10:28 error: no method named `insert` found for type `std::collections::hash::map::HashMap<T, std::collections::hash::set::HashSet<T>>` in the current scope
foo.rs:10             self.map.insert(node, HashSet::new());
                               ^~~~~~
foo.rs:10:22: 10:28 note: the method `insert` exists but the following trait bounds were not satisfied: `T : core::cmp::Eq`, `T : core::hash::Hash`
error: aborting due to 2 previous errors
