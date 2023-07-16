 rust
    pub fn new(enabled: bool) -> DepGraph {
        DepGraph {
            data: Rc::new(DepGraphData {
                thread: DepGraphThreadData::new(enabled),
                previous_work_products: RefCell::new(FnvHashMap()),
                work_products: RefCell::new(FnvHashMap()),
                #[cfg(debug_assertions)] // what I want to write, but can't
                forbidden: RefCell::new(Vec::new()),
            })
        }
    }
