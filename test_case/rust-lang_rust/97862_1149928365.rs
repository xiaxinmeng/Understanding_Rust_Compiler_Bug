plain
    Checking tracing-tree v0.2.0
    Checking chalk-solve v0.80.0
    Checking rustc_log v0.0.0 (/checkout/compiler/rustc_log)
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error[E0277]: can't compare `u32` with `&u32`
    |
    |
227 |             if start > end || current.map_or(false, |x| x + 1 >= start) {
    |                                                               ^^ no implementation for `u32 < &u32` and `u32 > &u32`
    |
    = help: the trait `PartialOrd<&u32>` is not implemented for `u32`
    = help: the following other types implement trait `PartialOrd<Rhs>`:
              f64
              i128
              i16
              i32
