
error[E0308]: mismatched types
   --> f.rs:6:17
    |
5   |     a.insert(0, 1);
    |              -  - this is of type `{integer}`, which makes `a` to be inferred as `BTreeMap<{integer}, {integer}>`
    |              |
    |              this is of type `{integer}`, which makes `a` to be inferred as `BTreeMap<{integer}, {integer}>`
6   |     a.insert(0, "foo");
    |       ------    ^^^^^ expected integer, found `&str`
    |       |
    |       arguments to this function are incorrect
    |
note: associated function defined here
   --> /home/gh-estebank/rust/library/alloc/src/collections/btree/map.rs:955:12
    |
955 |     pub fn insert(&mut self, key: K, value: V) -> Option<V>
    |            ^^^^^^
