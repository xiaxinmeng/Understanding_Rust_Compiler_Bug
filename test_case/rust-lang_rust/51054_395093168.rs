
njn, Zoxc: every query result needs to be hashed, otherwise incremental compilation could not recover from false positive. We started out with just hashing the HIR but re-use was very poor
njn, Zoxc: I think there's a bug report for everything that we forgot to hash properly :)  
njn, Zoxc: https://github.com/rust-lang/rust/issues/47389 contains some background on why spans are hashed and how things could be improved a little.
