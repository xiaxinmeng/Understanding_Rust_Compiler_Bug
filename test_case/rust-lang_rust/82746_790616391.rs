rust
use std::collections::HashMap;

pub trait Rule {
    type DependencyKey;
}

pub struct RuleEdges<R: Rule> {
  dependencies: R::DependencyKey,
}
type RuleDependencyEdges<R> = HashMap<u32, RuleEdges<R>>;
