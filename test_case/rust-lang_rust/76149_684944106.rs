rust
fn use_owner(mut owner: Owner) {
    let o = owner.share();
    let _v = match o {
        Ok(v) => v,
        Err(()) => {
            drop(o);
            owner.do_something_else();
            return;
        }
    };
}
