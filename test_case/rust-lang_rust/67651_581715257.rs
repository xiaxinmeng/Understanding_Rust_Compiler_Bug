rust
mod queue {
    pub trait TryMerge {}
    
    impl TryMerge for () {}
    
    pub fn work_queue<W, C, WF>(work_fn: W)
    where
        W: Fn(C) -> WF,
        C: TryMerge,
    {}
}

impl queue::TryMerge for () {}

fn main() {
    queue::work_queue(move |_: ()| {
        async {}
    })
}
