 rust
/// Insert a value into the map
pub fn insert<K:Copy + Eq + Ord,V:Copy>(m: Treemap<K, V>, k: K, v: V) -> Treemap<K, V> {
    @match m {
        @Empty => Node(@k, @v, @Empty, @Empty),
        @Node(@copy kk, vv, left, right) => cond!(
            | k <  kk { Node(@kk, vv, insert(left, k, v), right) }
            | k == kk { Node(@kk, @v, left, right)               }
            _         { Node(@kk, vv, left, insert(right, k, v)) }
        )
    }
}

/// Find a value based on the key
pub fn find<K:Eq + Ord,V:Copy>(m: Treemap<K, V>, k: K) -> Option<V> {
    match *m {
        Empty => None,
        Node(@ref kk, @copy v, left, right) => cond!(
            | k == *kk { Some(v)        }
            | k <  *kk { find(left, k)  }
            _          { find(right, k) }
        )
    }
}
