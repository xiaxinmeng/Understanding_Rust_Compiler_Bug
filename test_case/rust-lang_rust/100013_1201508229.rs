rust
#![feature(generic_associated_types)]

pub trait FutureIterator {
    type Future<'s, 'cx>: Send
    where
        's: 'cx;
}

fn call_2<I: FutureIterator>() -> impl Send {
    async { // a generator checked for autotrait impl `Send`
        let x = None::<I::Future<'_, '_>>; // a type referencing GAT
        async {}.await; // a yield point
    }
}
