plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0412]: cannot find type `Cell` in this scope
    --> compiler/rustc_middle/src/ty/context.rs:1269:25
     |
1269 |             static TLV: Cell<usize> = Cell::new(0);
     |
help: consider importing one of these items
     |
1268 |         use core::cell::Cell;
1268 |         use core::cell::Cell;
     |
1268 |         use std::cell::Cell;
     |

error[E0433]: failed to resolve: use of undeclared type `Cell`
    --> compiler/rustc_middle/src/ty/context.rs:1269:39
     |
1269 |             static TLV: Cell<usize> = Cell::new(0);
     |                                       ^^^^ use of undeclared type `Cell`
help: consider importing one of these items
     |
1268 |         use core::cell::Cell;
     |
     |
1268 |         use std::cell::Cell;
     |

    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: mismatched types
    --> compiler/rustc_middle/src/ty/context.rs:1273:75
     |
1273 |         pub(super) fn set_tlv<F: FnOnce() -> R, R>(value: usize, f: F) -> R {
     |                       -------                   -                         ^ expected type parameter `R`, found `()`
     |                       |                         this type parameter
     |                       |                         this type parameter
     |                       implicitly returns `()` as its body has no tail or `return` expression
     = note: expected type parameter `R`
                     found unit type `()`

Some errors have detailed explanations: E0308, E0412, E0433.
