
#[inline(always)]
fn doit(...) {
    if (load(...) < 0) {
        return;
    }
    doit_slow(...);
}

fn doit_slow() { ... }
