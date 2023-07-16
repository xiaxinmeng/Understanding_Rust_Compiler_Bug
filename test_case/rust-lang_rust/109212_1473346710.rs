rust
let is_staged_api = self.lookup_stability(def_id.krate.as_def_id()).is_some();
if !is_staged_api {
    return EvalResult::Allow;
}
