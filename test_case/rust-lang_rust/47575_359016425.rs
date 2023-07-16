rust
// FIXME(eddyb) move to rustc_data_structures.
#[derive(Clone)]
pub struct SparseBitSet<I: Idx> {
    map: BTreeMap<u32, u128>,
    _marker: PhantomData<I>,
}

fn key_and_mask<I: Idx>(index: I) -> (u32, u128) {
    let index = index.index();
    let key = index / 128;
    let key_u32 = key as u32;
    assert_eq!(key_u32 as usize, key);
    (key_u32, 1 << (index % 128))
}

impl<I: Idx> SparseBitSet<I> {
    pub fn new() -> Self {
        SparseBitSet {
            map: BTreeMap::new(),
            _marker: PhantomData
        }
    }

    pub fn capacity(&self) -> usize {
        self.map.len() * 128
    }

    pub fn contains(&self, index: I) -> bool {
        let (key, mask) = key_and_mask(index);
        self.map.get(&key).map_or(false, |bits| (bits & mask) != 0)
    }

    pub fn insert(&mut self, index: I) -> bool {
        let (key, mask) = key_and_mask(index);
        let bits = self.map.entry(key).or_insert(0);
        let old_bits = *bits;
        let new_bits = old_bits | mask;
        *bits = new_bits;
        new_bits != old_bits
    }

    pub fn remove(&mut self, index: I) -> bool {
        let (key, mask) = key_and_mask(index);
        if let Some(bits) = self.map.get_mut(&key) {
            let old_bits = *bits;
            let new_bits = old_bits & !mask;
            *bits = new_bits;
            // FIXME(eddyb) maybe remove entry if now `0`.
            new_bits != old_bits
        } else {
            false
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = I> + 'a {
        self.map.iter().flat_map(|(&key, &bits)| {
            let base = key as usize * 128;
            (0..128).filter_map(move |i| {
                if (bits & (1 << i)) != 0 {
                    Some(I::new(base + i))
                } else {
                    None
                }
            })
        })
    }
}
