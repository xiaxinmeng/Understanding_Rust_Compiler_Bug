rust
if self.track_individual_lifetime_uses && !self.lifetime_uses.contains_key(&def_id) {
    self.lifetime_uses.insert(def_id, LifetimeUseSet::One(lifetime_ref));
} else {
    self.lifetime_uses.insert(def_id, LifetimeUseSet::Many);
}
