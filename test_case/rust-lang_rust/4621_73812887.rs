
macro_rules! defn {
    // A match without return type
    ($n:ident) => (
            // Adding pub here results in: 
            //   error: expected one of `extern`, `fn`, or `unsafe`, found `pub`
            /* pub */ fn $n (&self) {
              println!("hi");
        })
}

pub struct Struct;

impl Struct {
  // Adding pub here doesn't give an error, but the
  // function ends up not being public.
  pub defn!(f);
}
