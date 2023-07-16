rust
#![feature(generic_const_exprs)]
trait Foo {}

impl Foo for () where Bar<0>: {}

struct Bar<const T: usize>
where
    [(); T + 0 ]:,
    [(); T + 0 ]:,
    [(); T + 0 ]:,
    [(); T + 0 ]:,
    [(); T + 0 ]:,
    [(); T + 0 ]:,
    [(); T + 0 ]:,
    [(); T + 0 ]:,
    [(); T + 0 ]:, {}
