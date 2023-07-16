 rust
let orig_xray = self.xray_context;
self.xray_context = Xray;
match self.resolve_path(path, TypeNS, true, visitor) {
    Some(def) => {
        // Found it, but it's private
    }
    None => {
        // Straight-up doesn't exist
    }
}
let self.xray_context = orig_xray;
