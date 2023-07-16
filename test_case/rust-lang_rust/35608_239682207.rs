 rust
/// A `FullMap<K, V>` is a complete mapping from `K`s to `V`s - every `K` corresponds
/// to a `V`.
struct FullMap<K, V> {
    // The value corresponding to every key not in exceptions
    default: V,
    exceptions: HashMap<K, V>
}

impl<K, V> FullMap<K, V> {
    fn get<'a>(&'a self, key: &K) -> &'a V {
        self.exceptions.get(key).unwrap_or(&self.default)
    }

    fn insert(&mut self, key: K, value: V) -> V {
        if value == self.default {
            self.exceptions.remove(&key).unwrap_or(value)
        } else {
            self.exceptions.insert(key, value).unwrap_or_else(|| self.default.clone())
        }
    }
}

impl<K, V: Lattice> Lattice for FullMap<K, V> {
    fn bottom() -> Self {
        FullMap {
            default: bottom(),
            exceptions: HashMap::new()
        }
    }

    fn join(&mut self, other: &FullMap<K, V>) -> bool {
        // This could definitely be optimized when defaults are known to be top or bottom
        let old_default = self.default.clone();
        let mut changed = self.default.join(&other.default);
        for (k, v) in &mut self.exceptions {
            if !other.exceptions.contains_key(&k) {
                changed |= v.join(&other.default);
                // TODO: for efficiency, remove keys here equal to the default
            }
        }

        for (k, v) in &other.exceptions {
            match self.exceptions.entry(k.clone()) {
                Occupied(occupied) => {
                    changed |= occupied.get_mut().join(v);
                    if *occupied.get() == self.default {
                        occupied.remove();
                    }
                },
                Vacant(vacant) => {
                    let mut cur_v = old_default.clone();
                    changed |= cur_v.join(v);
                    if cur_v != self.default {
                        vacant.insert(cur_v)
                    }
                }
            }
        }
        changed
    }
}

struct CsLattice {
    inner: FullMap<Lvalue, WTopBottom<ConstVal>>
}
