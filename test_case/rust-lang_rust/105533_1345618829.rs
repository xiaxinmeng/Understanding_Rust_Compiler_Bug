plain
    Checking addr2line v0.17.0
error: associated function has missing stability attribute
   --> library/std/src/collections/hash/map.rs:904:5
    |
904 | /     pub fn get_or_insert<Q: ?Sized>(&mut self, k: K, callback: fn() -> V) -> &V
905 | |     where
906 | |         K: Borrow<Q>,
907 | |         Q: Hash + Eq,
916 | |         }
917 | |     }
    | |_____^


error[E0308]: mismatched types
    --> library/std/src/collections/hash/map.rs:909:38
     |
732  | impl<K, V, S> HashMap<K, V, S>
...
...
909  |         match self.base.contains_key(k) {
     |                         ------------ ^ expected `&Q`, found type parameter `K`
     |                         arguments to this function are incorrect
     |
     = note:   expected reference `&Q`
             found type parameter `K`
             found type parameter `K`
note: associated function defined here
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/map.rs:1331:12
     |
1331 |     pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool

error[E0308]: mismatched types
    --> library/std/src/collections/hash/map.rs:910:35
     |
     |
732  | impl<K, V, S> HashMap<K, V, S>
...
...
910  |             true => self.base.get(k).unwrap(),
     |                               --- ^ expected `&Q`, found type parameter `K`
     |                               arguments to this function are incorrect
     |
     = note:   expected reference `&Q`
             found type parameter `K`
             found type parameter `K`
note: associated function defined here
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/map.rs:1217:12
     |
1217 |     pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>

error[E0308]: mismatched types
    --> library/std/src/collections/hash/map.rs:914:31
     |
     |
732  | impl<K, V, S> HashMap<K, V, S>
...
...
914  |                 self.base.get(k).unwrap()
     |                           --- ^ expected `&Q`, found type parameter `K`
     |                           arguments to this function are incorrect
     |
     = note:   expected reference `&Q`
             found type parameter `K`
             found type parameter `K`
note: associated function defined here
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/src/map.rs:1217:12
     |
1217 |     pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>

For more information about this error, try `rustc --explain E0308`.
error: could not compile `std` due to 4 previous errors
Build completed unsuccessfully in 0:02:33
