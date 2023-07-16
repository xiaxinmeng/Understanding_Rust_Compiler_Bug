plain
   Compiling rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
[RUSTC-TIMING] rustc_feature test:false 1.383
[RUSTC-TIMING] rustc_parse_format test:false 1.592
   Compiling gsgdt v0.1.2
error[E0599]: no method named `to_result` found for struct `TyAndLayout<'_, Ty>` in the current scope
    --> compiler/rustc_target/src/abi/mod.rs:1338:53
     |
1207 | pub struct TyAndLayout<'a, Ty> {
     | ------------------------------ method `to_result` not found for this
...
1338 |                 if *count > 0 && !self.field(cx, 0).to_result()?.might_permit_raw_init(cx, zero)? {
     |                                                     ^^^^^^^^^ method not found in `TyAndLayout<'_, Ty>`

error[E0277]: the `?` operator can only be used in a method that returns `Result` or `Option` (or another type that implements `FromResidual`)
    --> compiler/rustc_target/src/abi/mod.rs:1338:64
     |
1305 | /     pub fn might_permit_raw_init<C>(self, cx: &C, zero: bool) -> bool
1306 | |     where
1307 | |         Self: Copy,
1308 | |         Ty: TyAbiInterface<'a, C>,
...    |
1338 | |                 if *count > 0 && !self.field(cx, 0).to_result()?.might_permit_raw_init(cx, zero)? {
     | |                                                                ^ cannot use the `?` operator in a method that returns `bool`
1354 | |         true
1355 | |     }
1355 | |     }
     | |_____- this function should return `Result` or `Option` to accept `?`
     |
     = help: the trait `FromResidual<_>` is not implemented for `bool`

error[E0277]: the `?` operator can only be used in a method that returns `Result` or `Option` (or another type that implements `FromResidual`)
    --> compiler/rustc_target/src/abi/mod.rs:1338:97
     |
1305 | /     pub fn might_permit_raw_init<C>(self, cx: &C, zero: bool) -> bool
1306 | |     where
1307 | |         Self: Copy,
1308 | |         Ty: TyAbiInterface<'a, C>,
...    |
1338 | |                 if *count > 0 && !self.field(cx, 0).to_result()?.might_permit_raw_init(cx, zero)? {
     | |                                                                                                 ^ cannot use the `?` operator in a method that returns `bool`
1354 | |         true
1355 | |     }
1355 | |     }
     | |_____- this function should return `Result` or `Option` to accept `?`
     |
     = help: the trait `FromResidual<_>` is not implemented for `bool`
   Compiling tracing-serde v0.1.2
   Compiling rls-span v0.5.3
[RUSTC-TIMING] serde test:false 4.636
error[E0308]: mismatched types
error[E0308]: mismatched types
    --> compiler/rustc_target/src/abi/mod.rs:1340:28
     |
1305 |     pub fn might_permit_raw_init<C>(self, cx: &C, zero: bool) -> bool
     |                                                                  ---- expected `_` because of return type
1340 |                     return Ok(false);
     |                            ^^^^^^^^^ expected `bool`, found enum `Result`
     |
     = note: expected type `bool`
