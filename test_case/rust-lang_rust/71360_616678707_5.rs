compile_fail,E0432
> #![feature(non_portable_conversion)]
> 
> // These two never exist at the same time:
> use std::target::PointerWidthGe64From;
> use std::target::PointerWidthLe32From;
> // error[E0432]: unresolved import
> 