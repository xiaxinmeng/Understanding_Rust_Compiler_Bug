rust
let a: Iterator + TrustedLen + TrustedUsizeLen = ...;
let b: Iterator + TrustedLen + TrustedUsizeLen = ...;
let chained: Iterator + TrustedLen  = a.chain(b);
let length_recovered: Iterator + TrustedLen + TrustedUsizeLen = chained.take(usize::MAX)
