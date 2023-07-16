Rust
struct Owner {
    to_be_shared: String,
}

impl Owner {
    fn share(&mut self) -> Result<Box<dyn ToString + '_>, ()> {
        Ok(Box::new(&self.to_be_shared) as Box<dyn ToString>)
    }
    fn do_something_else(&mut self) {}
}

fn use_owner(mut owner: Owner) {
    // disable drop code by leaking
    let _v = match owner.share().map(Box::leak) {
        Ok(v) => Ok::<_, ()>(v),
        Err(()) => {
            owner.do_something_else();
            return;
        }
    }
    .map(|leaked| unsafe { Box::from_raw(leaked as *mut _) }); // undo the leak right away, so the drop code runs after the match
}

fn main() {
    let owner = Owner {
        to_be_shared: String::new(),
    };
    use_owner(owner);
}
