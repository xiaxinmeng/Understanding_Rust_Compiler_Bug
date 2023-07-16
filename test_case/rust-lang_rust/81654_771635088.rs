
warning: a method with this name may be added to the standard library in the future
    --> src/atof/algorithm/math.rs:1909:34
     |
1909 | const ALGORITHM_D_B: Wide = 1 << Limb::BITS;
     |                                  ^^^^^^^^^^
     |
     = note: `#[warn(unstable_name_collisions)]` on by default
     = warning: once this method is added to the standard library, the ambiguity may cause an error or change in behavior!
     = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
     = help: call with fully qualified syntax `util::num::Integer::BITS(...)` to keep using the current method
