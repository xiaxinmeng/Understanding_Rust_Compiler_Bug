
use std::path::Path;
struct A {
    pub func: fn(check: bool, a: &Path, b: Option<&Path>),
}
const MY_A: A = A {
    func: |check, a, b| {
        if check {
            let _ = ();
        } else {
            // put the if let on another line
            if let Some(parent) = b.and_then(|p| p.parent()) {
                let _ = ();
            }
        }
    },
};
