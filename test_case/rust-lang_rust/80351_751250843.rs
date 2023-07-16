
struct Source {
    // essentially AnyMap
}

trait Query<'a> {
    type Result: 'a;
    fn execute(source: &'a Source) -> Self::Result;
}

fn enqueue<Q: for<'q> Query<'q>, F: 'static + for<'r> FnOnce(<Q as Query<'r>>::Result)>(f: F) {
    let _box: Box<dyn FnOnce(&Source)> = Box::new(move |source| f(Q::execute(source)));
    // Store _box in queue
}
