 rust
trait Trace {
    fn trace(&self, gc_info: &mut GcInfo);
}
impl<T: Trace> Trace for Gc<T> {
    fn trace(&self, gc_info: &mut GcInfo) {
         if gc_info.mark_reachable(self) { // returns true if already traced
             x.borrow().trace(gc_info);
         }
    }
}

impl<T: Trace> Trace for RefCell<T> {
    fn trace(&self, gc_info: &mut GcInfo) {
        self.value.trace(gc_info);
    }
}

impl Trace for int { fn trace(&self, _: &mut GcInfo) {} }
impl<T> Trace for *T { fn trace(&self, _: &mut GcInfo) {} }
// and similarly for the other basic types (I don't think we can/should
// impose any particular tracing semantics on `*`?)

// these are registered as roots for (precise) scanning separately, or have
// been registered to be included in any conservative scans (and have 
// proper impls here). (The latter is probably better; see below.)
impl<T> Trace for Uniq<T> { fn trace(&self, _: &mut GcInfo) {} }
impl<T> Trace for Vec<T> { fn trace(&self, _: &mut GcInfo) {} }
impl<T> Trace for Rc<T> { fn trace(&self, _: &mut GcInfo) {} }

// etc.
