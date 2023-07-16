rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
macro_rules! cons {
    ($ head : ident) => (($ head, ())) ;
    ($ head : ident, $ ($ tail : ident), *) =>
    (($ head, cons ! ($ ($ tail), *))) ;
}

fn blah() {
    let (a,
         (b,
          (c,
           (d,
            (e,
             (f,
              (g,
               (h,
                (i,
                 (j,
                  (k,
                   (l,
                    (m,
                     (n,
                      (o,
                       (p,
                        (q,
                         (r,
                          (s,
                           (t,
                            (u, (v, (w, (x, (z, ()))))))))))))))))))))))))) =
        { ::std::rt::begin_panic("not yet implemented") };
}
