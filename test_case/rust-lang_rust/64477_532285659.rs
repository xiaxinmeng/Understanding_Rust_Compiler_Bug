rust
use std::cell::RefCell;

fn foo() -> impl Send {
    async {
        let x = RefCell::new(String::new());
        match x {
            ref z => {
                drop(z);
                async move {
                    x
                }.await
            }
        }
    }
}
