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
    { // this scope is now required to truly sever the link between the locals and `owner`
        let res = owner.share();
        let _v = match res {
            Ok(v) => v,
            Err(()) => {
                drop(res);
                owner.do_something_else();
                return;
            }
        };
    }
    // this is the added line, which I would expect to work without the extra scope.
    owner.do_something_else();
}

fn main() {
    let owner = Owner {
        to_be_shared: String::new(),
    };
    use_owner(owner);
}
