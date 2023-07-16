
let mut live_on_entry: Map<Block, BitSet> = ...;

while not yet at fixed point {
    for each block B {
        let mut bits = union of live_on_entry(S) for each successor S of the block B;
        for each location L in B (going backwards) {
            bits = bits - kill(L) + gen(L);
        }
        live_on_entry(B) = bits;
    }
}
