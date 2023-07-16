rust
if let Some(generics) = imp.trait_.and_then(|t| t.generics()) {
    for typaram in generics {
        if let Some(did) = typaram.def_id() {
            if did.is_local() && !self.retained.contains(did) {
                return None;
            }
        }
    }
}
