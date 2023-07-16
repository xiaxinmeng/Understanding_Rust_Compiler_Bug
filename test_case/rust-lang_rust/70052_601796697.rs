
error: cannot specialize on trait `core::marker::Copy`
    --> src/raw/mod.rs:1047:1
     |
1047 | / impl<T: Copy> RawTableClone for RawTable<T> {
1048 | |     #[cfg_attr(feature = "inline-more", inline)]
1049 | |     unsafe fn clone_from_spec(&mut self, source: &Self, _on_panic: impl FnMut(&mut Self)) {
1050 | |         source
...    |
1060 | |     }
1061 | | }
     | |_^

error: cannot specialize on trait `core::cmp::Eq`
   --> src/map.rs:225:9
    |
225 | /         impl<K: Clone, V: Clone, S> HashClone<S> for HashMap<K, V, S>
226 | |         where
227 | |             K: Eq + Hash,
228 | |             S: BuildHasher,
...   |
234 | |             }
235 | |         }
    | |_________^

error: cannot specialize on trait `core::hash::Hash`
   --> src/map.rs:225:9
    |
225 | /         impl<K: Clone, V: Clone, S> HashClone<S> for HashMap<K, V, S>
226 | |         where
227 | |             K: Eq + Hash,
228 | |             S: BuildHasher,
...   |
234 | |             }
235 | |         }
    | |_________^

error: cannot specialize on trait `core::hash::BuildHasher`
   --> src/map.rs:225:9
    |
225 | /         impl<K: Clone, V: Clone, S> HashClone<S> for HashMap<K, V, S>
226 | |         where
227 | |             K: Eq + Hash,
228 | |             S: BuildHasher,
...   |
234 | |             }
235 | |         }
    | |_________^
