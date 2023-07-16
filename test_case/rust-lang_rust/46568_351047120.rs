rust
trait TraitB { }
trait TraitA {
        fn a<'a, B>(b: B) where &'a B: TraitB;
}
