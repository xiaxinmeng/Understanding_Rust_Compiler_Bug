
error[E0277]: the trait bound `&B: TraitB` is not satisfied
  --> src/lib.rs:11:20
   |
1  | trait TraitA { fn a<B>(b: &B) where B: TraitB; }
   |                ------------------------------- required by `TraitA::a`
...
11 |         StructA::a(&b);
   |                    -^
   |                    |
   |                    the trait `TraitB` is not implemented for `&B`
   |                    help: consider removing 1 leading `&`-references
