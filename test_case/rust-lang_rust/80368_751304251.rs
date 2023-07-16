plain
    Checking rustc-rayon v0.3.0
    Checking tempfile v3.1.0
    Checking regex v1.3.9
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0425]: cannot find value `item` in this scope
   --> src/librustdoc/clean/utils.rs:629:8
    |
629 |     if item.is_fake() {
    |
help: consider importing one of these items
    |
    |
1   | use crate::core::source_map::symbol::sym_generated::item;
1   | use rustc_span::sym::item;
    |


error[E0425]: cannot find value `item` in this scope
   --> src/librustdoc/clean/utils.rs:633:15
    |
633 |     } else if item.def_id.is_top_level_module() {
    |
help: consider importing one of these items
    |
    |
1   | use crate::core::source_map::symbol::sym_generated::item;
1   | use rustc_span::sym::item;
    |


error[E0425]: cannot find value `item` in this scope
   --> src/librustdoc/clean/utils.rs:634:14
    |
634 |         Some(item.def_id)
    |
help: consider importing one of these items
    |
    |
1   | use crate::core::source_map::symbol::sym_generated::item;
1   | use rustc_span::sym::item;
    |

error: aborting due to 3 previous errors
