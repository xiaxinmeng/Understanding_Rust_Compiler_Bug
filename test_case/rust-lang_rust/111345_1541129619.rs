plain
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
error[E0308]: mismatched types
   --> compiler/rustc_hir/src/tests.rs:20:85
    |
20  |             StableCrateId::new(Symbol::intern("foo"), false, vec!["1".to_string()], kw::Empty);
    |             ------------------ arguments to this function are incorrect             ^^^^^^^^^ expected `&str`, found `Symbol`
note: associated function defined here
   --> /checkout/compiler/rustc_span/src/def_id.rs:149:12
    |
149 |     pub fn new(
149 |     pub fn new(
    |            ^^^

error[E0308]: mismatched types
   --> compiler/rustc_hir/src/tests.rs:22:85
    |
22  |             StableCrateId::new(Symbol::intern("foo"), false, vec!["2".to_string()], kw::Empty);
    |             ------------------ arguments to this function are incorrect             ^^^^^^^^^ expected `&str`, found `Symbol`
note: associated function defined here
   --> /checkout/compiler/rustc_span/src/def_id.rs:149:12
    |
149 |     pub fn new(
