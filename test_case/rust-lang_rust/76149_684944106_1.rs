rust
fn use_owner(mut owner: Owner) {
    let _v = match owner.share() {
        Ok(v) => v,
        e @ Err(()) => {
            drop(e);
            owner.do_something_else();
            return;
        }
    };
}
