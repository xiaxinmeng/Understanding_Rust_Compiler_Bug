rust
impl<'gcx> HashStable<StableHashingContext<'gcx>> for AdtDef {
    fn hash_stable<W: StableHasherResult>(&self,
                                          hcx: &mut StableHashingContext<'gcx>,
                                          hasher: &mut StableHasher<W>) {
        let cache_key = self as *const AdtDef as usize;
        ...
    }
}
