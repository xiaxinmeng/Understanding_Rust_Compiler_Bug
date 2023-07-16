rust
#![feature(negative_impls)]

trait IntSubset {}

impl <T> IntSubset for T where T: FixedSizeIntSubset + !ArbitrarySizeIntSubset {}
impl <T> IntSubset for T where T: !FixedSizeIntSubset + ArbitrarySizeIntSubset {}

trait FixedSizeIntSubset {}

impl<T: FixedSizeIntSubset> !ArbitrarySizeIntSubset for T {}

trait ArbitrarySizeIntSubset {}

impl<T: ArbitrarySizeIntSubset> !FixedSizeIntSubset for T {}

