plain
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0624]: associated function `opt_item_name` is private
    --> compiler/rustc_symbol_mangling/src/typeid/typeid_itanium_cxx_abi.rs:205:42
     |
205  |             let name = format!("{}", tcx.opt_item_name(def_id).unwrap());
     |
    ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:1994:5
     |
1994 |     fn opt_item_name(self, def_id: DefId) -> Option<Symbol> {
1994 |     fn opt_item_name(self, def_id: DefId) -> Option<Symbol> {
     |     ------------------------------------------------------- private associated function defined here

error[E0624]: associated function `opt_item_name` is private
    --> compiler/rustc_symbol_mangling/src/typeid/typeid_itanium_cxx_abi.rs:279:42
     |
279  |             let name = format!("{}", tcx.opt_item_name(*def_id).unwrap());
     |
    ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:1994:5
     |
1994 |     fn opt_item_name(self, def_id: DefId) -> Option<Symbol> {
1994 |     fn opt_item_name(self, def_id: DefId) -> Option<Symbol> {
     |     ------------------------------------------------------- private associated function defined here

error[E0624]: associated function `opt_item_name` is private
    --> compiler/rustc_symbol_mangling/src/typeid/typeid_itanium_cxx_abi.rs:290:42
     |
290  |             let name = format!("{}", tcx.opt_item_name(*def_id).unwrap());
     |
    ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:1994:5
     |
1994 |     fn opt_item_name(self, def_id: DefId) -> Option<Symbol> {
1994 |     fn opt_item_name(self, def_id: DefId) -> Option<Symbol> {
     |     ------------------------------------------------------- private associated function defined here

error[E0624]: associated function `opt_item_name` is private
    --> compiler/rustc_symbol_mangling/src/typeid/typeid_itanium_cxx_abi.rs:351:54
     |
351  |                         let name = format!("{}", tcx.opt_item_name(trait_ref.def_id).unwrap());
     |
    ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:1994:5
     |
1994 |     fn opt_item_name(self, def_id: DefId) -> Option<Symbol> {
