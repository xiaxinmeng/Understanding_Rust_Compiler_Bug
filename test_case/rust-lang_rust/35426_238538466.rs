 rust
struct Args {
    iter: vec::IntoIter<OsString>,
}

struct Env {
    iter: vec::IntoIter<(OsString, OsString)>,
}

impl !Send for Args {}
impl !Sync for Args {}

impl !Send for Env {}
impl !Sync for Env {}
