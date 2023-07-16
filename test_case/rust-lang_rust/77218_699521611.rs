rust
// Type aliases to replace complex data types used in the original code.
pub type Data = usize;
pub type Key = usize;
pub type DataRef<'c> = &'c Data;

pub struct Cache<'c> {
    cached_data_refs: Vec<DataRef<'c>>,
}

impl Cache<'_> {
    pub fn data_ref_list(&mut self, key: &Key) {
        for reference in vec![1, 2, 3] {
            // Following `if` statement is the reason for compiler crash. To trigger the crash it
            // is required to:
            //
            // * have `let` keyword missing from `if let Some(...)` expression
            // * have binding name in the `Some(...)` portion of the `if let Some()` shadow the
            //   name from the outer loop
            //
            if /*let*/ Some(reference) = self.cached_data_refs.get(*key) {
                unimplemented!()
            }
        }
    }
}
