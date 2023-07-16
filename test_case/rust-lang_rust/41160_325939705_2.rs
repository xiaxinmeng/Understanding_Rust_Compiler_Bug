rust

pub struct WrappedArray {
    pub val: [u16; 100]
}

impl WrappedArray {
    fn new() -> WrappedArray {
        WrappedArray {
            val: [0; 100],
        }
    }
}

impl Default for WrappedArray {
    #[inline(always)]
    fn default() -> WrappedArray {
        WrappedArray {
            val: [100; 100],
        }
    }
}

impl Copy for WrappedArray {}

impl Clone for WrappedArray {
    fn clone(&self) -> WrappedArray { *self }
}

pub fn boxed_struct() -> Box<WrappedArray> {
    // Makes stack copies:
    Box::new(WrappedArray::new())
    Box::new(WrappedArray::new().clone())
    Box::new(WrappedArray { val: [0; 100] })

   // Does not make stack copies:
   const A: WrappedArray = WrappedArray { val: [0; 100] };
   Box::new(A)
   Box::new(WrappedArray { val: [0; 100] }.clone())
   Box::new(WrappedArray::default())
   Box::new(*&WrappedArray { val: [0; 100] })
   // And anything with box syntax
}

