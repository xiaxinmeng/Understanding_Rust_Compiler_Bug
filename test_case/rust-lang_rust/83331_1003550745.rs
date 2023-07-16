rust
#[derive(Trait)]
#[cfg_attr(predicate, trait_helper)]
struct S;

#[test]
#[cfg_attr(predicate, ignore)]
fn my_test() {}
