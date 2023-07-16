
<source>:21:1: 23:2 error: not all trait items implemented, missing: `mul` [E0046]
<source>:21 impl<L: Diff> Mul for Multiplication<L> {
<source>:22     type Output = Multiplication<Self>;
<source>:23 }
<source>:21:1: 23:2 help: run `rustc --explain E0046` to see a detailed explanation
error: aborting due to previous error
