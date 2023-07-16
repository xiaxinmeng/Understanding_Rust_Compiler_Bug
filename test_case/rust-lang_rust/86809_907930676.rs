rust
#[deny(non_exhaustive_reachable_patterns)]
match NestedNonExhaustive::B {
   NestedNonExhaustive::A(NonExhaustiveEnum::Unit) => {}
   NestedNonExhaustive::A(_) => {}
   NestedNonExhaustive::B => {}
    _ => {}
}
