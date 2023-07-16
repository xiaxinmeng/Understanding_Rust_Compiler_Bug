rust
// FIXME(eddyb) move to rustc_data_structures.
#[derive(Clone)]
pub struct SparseBitSet<I: Idx> {
    chunk_bits: BTreeMap<u32, u128>,
    _marker: PhantomData<I>,
}

#[derive(Copy, Clone)]
pub struct SparseChunk<I> {
    key: u32,
    bits: u128,
    _marker: PhantomData<I>,
}

impl<I: Idx> SparseChunk<I> {
    pub fn one(index: I) -> Self {
        let index = index.index();
        let key_usize = index / 128;
        let key = key_usize as u32;
        assert_eq!(key as usize, key_usize);
        SparseChunk {
            key,
            bits: 1 << (index % 128),
            _marker: PhantomData
        }
    }

    pub fn any(&self) -> bool {
        self.bits != 0
    }

    pub fn iter(&self) -> impl Iterator<Item = I> {
        let base = self.key as usize * 128;
        let mut bits = self.bits;
        (0..128).map(move |i| {
            let current_bits = bits;
            bits >>= 1;
            (i, current_bits)
        }).take_while(|&(_, bits)| bits != 0)
          .filter_map(move |(i, bits)| {
            if (bits & 1) != 0 {
                Some(I::new(base + i))
            } else {
                None
            }
        })
    }
}

impl<I: Idx> SparseBitSet<I> {
    pub fn new() -> Self {
        SparseBitSet {
            chunk_bits: BTreeMap::new(),
            _marker: PhantomData
        }
    }

    pub fn capacity(&self) -> usize {
        self.chunk_bits.len() * 128
    }

    pub fn contains_chunk(&self, chunk: SparseChunk<I>) -> SparseChunk<I> {
        SparseChunk {
            bits: self.chunk_bits.get(&chunk.key).map_or(0, |bits| bits & chunk.bits),
            ..chunk
        }
    }

    pub fn insert_chunk(&mut self, chunk: SparseChunk<I>) -> SparseChunk<I> {
        if chunk.bits == 0 {
            return chunk;
        }
        let bits = self.chunk_bits.entry(chunk.key).or_insert(0);
        let old_bits = *bits;
        let new_bits = old_bits | chunk.bits;
        *bits = new_bits;
        let changed = new_bits ^ old_bits;
        SparseChunk {
            bits: changed,
            ..chunk
        }
    }

    pub fn remove_chunk(&mut self, chunk: SparseChunk<I>) -> SparseChunk<I> {
        if chunk.bits == 0 {
            return chunk;
        }
        let changed = match self.chunk_bits.entry(chunk.key) {
            Entry::Occupied(mut bits) => {
                let old_bits = *bits.get();
                let new_bits = old_bits & !chunk.bits;
                if new_bits == 0 {
                    bits.remove();
                } else {
                    bits.insert(new_bits);
                }
                new_bits ^ old_bits
            }
            Entry::Vacant(_) => 0
        };
        SparseChunk {
            bits: changed,
            ..chunk
        }
    }

    pub fn clear(&mut self) {
        self.chunk_bits.clear();
    }

    pub fn chunks<'a>(&'a self) -> impl Iterator<Item = SparseChunk<I>> + 'a {
        self.chunk_bits.iter().map(|(&key, &bits)| {
            SparseChunk {
                key,
                bits,
                _marker: PhantomData
            }
        })
    }

    pub fn contains(&self, index: I) -> bool {
        self.contains_chunk(SparseChunk::one(index)).any()
    }

    pub fn insert(&mut self, index: I) -> bool {
        self.insert_chunk(SparseChunk::one(index)).any()
    }

    pub fn remove(&mut self, index: I) -> bool {
        self.remove_chunk(SparseChunk::one(index)).any()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = I> + 'a {
        self.chunks().flat_map(|chunk| chunk.iter())
    }
}
