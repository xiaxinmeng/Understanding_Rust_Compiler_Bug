rust
match def {
    LateBoundAnon(..) | Static => {
        // These are anonymous lifetimes or lifetimes that are not declared.
    }

    Free(_, def_id) | LateBound(_, def_id) | EarlyBound(_, def_id) => {
        // A lifetime declared by the user.
        if !self.lifetime_uses.contains_key(&def_id) {
            self.lifetime_uses.insert(def_id, LifetimeUseSet::One(lifetime_ref));
        } else {
            self.lifetime_uses.insert(def_id, LifetimeUseSet::Many);
        }
    }
}
