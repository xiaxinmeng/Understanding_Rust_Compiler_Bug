 rust
fn run_tracing<T: Trace>(x: *(), gc_info: &mut GcInfo) {
     unsafe { (*(x as *T)).trace(gc_info) }
}

impl<T> Uniq<T> {
     fn new(x: T) -> Uniq<T> {
          // pseudo-Rust
          let ptr = malloc(size);
          *ptr = x;

           // get the appropriately monomorphised version of `run_tracing`
          register_with_gc(ptr, run_tracing::<T>);

          Uniq { ptr: ptr }
     }
}
