plain
2019-10-30T18:18:31.7092758Z    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
2019-10-30T18:18:34.5051483Z warning: unnecessary parentheses around type
2019-10-30T18:18:34.5052698Z    --> src/tools/clippy/clippy_lints/src/utils/mod.rs:242:71
2019-10-30T18:18:34.5053183Z     |
2019-10-30T18:18:34.5053960Z 242 | pub fn path_to_res(cx: &LateContext<'_, '_>, path: &[&str]) -> Option<(def::Res)> {
2019-10-30T18:18:34.5054976Z     |                                                                       ^^^^^^^^^^ help: remove these parentheses
2019-10-30T18:18:34.5055926Z     = note: `#[warn(unused_parens)]` on by default
2019-10-30T18:18:34.5059228Z 
2019-10-30T18:18:34.5260041Z warning: unnecessary parentheses around type
2019-10-30T18:18:34.5261005Z   --> src/tools/clippy/clippy_lints/src/mutex_atomic.rs:80:42
2019-10-30T18:18:34.5261005Z   --> src/tools/clippy/clippy_lints/src/mutex_atomic.rs:80:42
2019-10-30T18:18:34.5261457Z    |
2019-10-30T18:18:34.5261912Z 80 | fn get_atomic_name(ty: Ty<'_>) -> Option<(&'static str)> {
2019-10-30T18:18:34.5262644Z    |                                          ^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-30T18:18:35.9037125Z error[E0308]: mismatched types
2019-10-30T18:18:35.9037891Z    --> src/tools/clippy/clippy_lints/src/consts.rs:164:32
2019-10-30T18:18:35.9038369Z     |
2019-10-30T18:18:35.9038369Z     |
2019-10-30T18:18:35.9038916Z 164 |         LitKind::Float(ref is, FloatTy::F32) => Constant::F32(is.as_str().parse().unwrap()),
2019-10-30T18:18:35.9039891Z     |                                ^^^^^^^^^^^^ expected enum `syntax::ast::LitFloatType`, found enum `syntax::ast::FloatTy`
2019-10-30T18:18:35.9043110Z     = note: expected type `syntax::ast::LitFloatType`
2019-10-30T18:18:35.9043471Z                found type `syntax::ast::FloatTy`
2019-10-30T18:18:35.9047712Z 
2019-10-30T18:18:35.9082055Z error[E0308]: mismatched types
2019-10-30T18:18:35.9082055Z error[E0308]: mismatched types
2019-10-30T18:18:35.9082419Z    --> src/tools/clippy/clippy_lints/src/consts.rs:165:32
2019-10-30T18:18:35.9082655Z     |
2019-10-30T18:18:35.9083026Z 165 |         LitKind::Float(ref is, FloatTy::F64) => Constant::F64(is.as_str().parse().unwrap()),
2019-10-30T18:18:35.9083480Z     |                                ^^^^^^^^^^^^ expected enum `syntax::ast::LitFloatType`, found enum `syntax::ast::FloatTy`
2019-10-30T18:18:35.9084113Z     = note: expected type `syntax::ast::LitFloatType`
2019-10-30T18:18:35.9084574Z                found type `syntax::ast::FloatTy`
2019-10-30T18:18:35.9090930Z 
2019-10-30T18:18:35.9090930Z 
2019-10-30T18:18:35.9793052Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:35.9794166Z     |
2019-10-30T18:18:35.9794166Z     |
2019-10-30T18:18:35.9794500Z 166 |         LitKind::FloatUnsuffixed(ref is) => match ty.expect("type of float is known").kind {
2019-10-30T18:18:35.9795099Z 
2019-10-30T18:18:35.9795099Z 
2019-10-30T18:18:36.2423214Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:36.2423862Z     |
2019-10-30T18:18:36.2424289Z 292 |                     LitKind::FloatUnsuffixed(_) => {
2019-10-30T18:18:36.2424721Z     |                              ^^^^^^^^^^^^^^^ variant or associated item not found in `syntax::ast::LitKind`
2019-10-30T18:18:36.2428380Z 
2019-10-30T18:18:36.2428380Z 
2019-10-30T18:18:36.8985908Z error[E0308]: mismatched types
2019-10-30T18:18:36.8986812Z   --> src/tools/clippy/clippy_lints/src/approx_const.rs:65:27
2019-10-30T18:18:36.8987150Z    |
2019-10-30T18:18:36.8987501Z 65 |         LitKind::Float(s, FloatTy::F32) => check_known_consts(cx, e, s, "f32"),
2019-10-30T18:18:36.8987977Z    |                           ^^^^^^^^^^^^ expected enum `syntax::ast::LitFloatType`, found enum `syntax::ast::FloatTy`
2019-10-30T18:18:36.8988593Z    = note: expected type `syntax::ast::LitFloatType`
2019-10-30T18:18:36.8989068Z               found type `syntax::ast::FloatTy`
2019-10-30T18:18:36.8994303Z 
2019-10-30T18:18:36.9029163Z error[E0308]: mismatched types
2019-10-30T18:18:36.9029163Z error[E0308]: mismatched types
2019-10-30T18:18:36.9029509Z   --> src/tools/clippy/clippy_lints/src/approx_const.rs:66:27
2019-10-30T18:18:36.9030014Z    |
2019-10-30T18:18:36.9030375Z 66 |         LitKind::Float(s, FloatTy::F64) => check_known_consts(cx, e, s, "f64"),
2019-10-30T18:18:36.9031006Z    |                           ^^^^^^^^^^^^ expected enum `syntax::ast::LitFloatType`, found enum `syntax::ast::FloatTy`
2019-10-30T18:18:36.9031663Z    = note: expected type `syntax::ast::LitFloatType`
2019-10-30T18:18:36.9031978Z               found type `syntax::ast::FloatTy`
2019-10-30T18:18:36.9036940Z 
2019-10-30T18:18:36.9036940Z 
2019-10-30T18:18:36.9113057Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:36.9114163Z    |
2019-10-30T18:18:36.9114163Z    |
2019-10-30T18:18:36.9114543Z 67 |         LitKind::FloatUnsuffixed(s) => check_known_consts(cx, e, s, "f{32, 64}"),
2019-10-30T18:18:36.9119693Z 
2019-10-30T18:18:36.9119693Z 
2019-10-30T18:18:37.4680163Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:37.4680929Z    |
2019-10-30T18:18:37.4680929Z    |
2019-10-30T18:18:37.4681294Z 46 |             if let LitKind::Float(sym, _) | LitKind::FloatUnsuffixed(sym) = lit.node;
2019-10-30T18:18:37.4685744Z 
2019-10-30T18:18:37.4685744Z 
2019-10-30T18:18:37.8438941Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:37.8439711Z     |
2019-10-30T18:18:37.8440217Z 376 |             LitKind::Float(..) | LitKind::FloatUnsuffixed(..) => {
2019-10-30T18:18:37.8440699Z     |                                           ^^^^^^^^^^^^^^^ variant or associated item not found in `syntax::ast::LitKind`
2019-10-30T18:18:37.8444512Z 
2019-10-30T18:18:37.8444512Z 
2019-10-30T18:18:38.5722639Z error[E0599]: no method named `ty_to_string` found for type `syntax::ast::IntTy` in the current scope
2019-10-30T18:18:38.5723358Z    --> src/tools/clippy/clippy_lints/src/misc_early.rs:485:46
2019-10-30T18:18:38.5723968Z     |
2019-10-30T18:18:38.5724328Z 485 |                 LitIntType::Signed(ty) => ty.ty_to_string(),
2019-10-30T18:18:38.5725262Z     |                                              ^^^^^^^^^^^^ help: there is a method with a similar name: `val_to_string`
2019-10-30T18:18:38.5882507Z error[E0599]: no method named `ty_to_string` found for type `syntax::ast::UintTy` in the current scope
2019-10-30T18:18:38.5882885Z    --> src/tools/clippy/clippy_lints/src/misc_early.rs:486:48
2019-10-30T18:18:38.5883174Z     |
2019-10-30T18:18:38.5883174Z     |
2019-10-30T18:18:38.5883524Z 486 |                 LitIntType::Unsigned(ty) => ty.ty_to_string(),
2019-10-30T18:18:38.5884154Z     |                                                ^^^^^^^^^^^^ help: there is a method with a similar name: `val_to_string`
2019-10-30T18:18:38.5888350Z 
2019-10-30T18:18:38.6077365Z error[E0599]: no method named `ty_to_string` found for type `syntax::ast::LitFloatType` in the current scope
2019-10-30T18:18:38.6078209Z     |
2019-10-30T18:18:38.6078209Z     |
2019-10-30T18:18:38.6078519Z 547 |             let suffix = float_ty.ty_to_string();
2019-10-30T18:18:38.6078945Z     |                                   ^^^^^^^^^^^^ method not found in `syntax::ast::LitFloatType`
2019-10-30T18:18:38.6082970Z 
2019-10-30T18:18:38.9746478Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:38.9748037Z    |
2019-10-30T18:18:38.9748608Z 93 | ...                   LitKind::Int(..) | LitKind::Float(..) | LitKind::FloatUnsuffixed(..) => {
2019-10-30T18:18:38.9749274Z    |                                                                        ^^^^^^^^^^^^^^^ variant or associated item not found in `syntax::ast::LitKind`
2019-10-30T18:18:38.9752239Z 
2019-10-30T18:18:38.9752239Z 
2019-10-30T18:18:39.0911697Z [RUSTC-TIMING] cargo_metadata test:false 19.633
2019-10-30T18:18:39.3096224Z error[E0277]: `syntax::ast::UintTy` doesn't implement `std::fmt::Display`
2019-10-30T18:18:39.3097559Z    --> src/tools/clippy/clippy_lints/src/transmute.rs:393:47
2019-10-30T18:18:39.3098255Z     |
2019-10-30T18:18:39.3098921Z 393 | ...                   arg.as_ty(ast::UintTy::U32)
2019-10-30T18:18:39.3099988Z     |                                 ^^^^^^^^^^^^^^^^ `syntax::ast::UintTy` cannot be formatted with the default formatter
2019-10-30T18:18:39.3101129Z     = help: the trait `std::fmt::Display` is not implemented for `syntax::ast::UintTy`
2019-10-30T18:18:39.3101672Z     = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-10-30T18:18:39.3106178Z 
2019-10-30T18:18:39.3106178Z 
2019-10-30T18:18:39.4411506Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:39.4415265Z      |
2019-10-30T18:18:39.4415265Z      |
2019-10-30T18:18:39.4415691Z 1189 |                     LitKind::Int(_, LitIntType::Unsuffixed) | LitKind::FloatUnsuffixed(_) => {},
2019-10-30T18:18:39.4421168Z 
2019-10-30T18:18:39.7501260Z error: aborting due to 15 previous errors
2019-10-30T18:18:39.7505198Z 
2019-10-30T18:18:39.7511000Z Some errors have detailed explanations: E0277, E0308, E0599.
2019-10-30T18:18:39.7511000Z Some errors have detailed explanations: E0277, E0308, E0599.
2019-10-30T18:18:39.7516849Z For more information about an error, try `rustc --explain E0277`.
2019-10-30T18:18:39.8258074Z [RUSTC-TIMING] clippy_lints test:false 8.110
2019-10-30T18:18:39.8284384Z error: could not compile `clippy_lints`.
2019-10-30T18:18:39.8300703Z warning: build failed, waiting for other jobs to finish...
2019-10-30T18:18:41.9976566Z warning: unnecessary parentheses around type
2019-10-30T18:18:41.9977804Z    --> src/tools/clippy/clippy_lints/src/utils/mod.rs:242:71
2019-10-30T18:18:41.9978268Z     |
2019-10-30T18:18:41.9978792Z 242 | pub fn path_to_res(cx: &LateContext<'_, '_>, path: &[&str]) -> Option<(def::Res)> {
2019-10-30T18:18:41.9979400Z     |                                                                       ^^^^^^^^^^ help: remove these parentheses
2019-10-30T18:18:41.9980338Z     = note: `#[warn(unused_parens)]` on by default
2019-10-30T18:18:41.9980554Z 
2019-10-30T18:18:42.0142167Z warning: unnecessary parentheses around type
2019-10-30T18:18:42.0143100Z   --> src/tools/clippy/clippy_lints/src/mutex_atomic.rs:80:42
2019-10-30T18:18:42.0143100Z   --> src/tools/clippy/clippy_lints/src/mutex_atomic.rs:80:42
2019-10-30T18:18:42.0143999Z    |
2019-10-30T18:18:42.0145060Z 80 | fn get_atomic_name(ty: Ty<'_>) -> Option<(&'static str)> {
2019-10-30T18:18:42.0145915Z    |                                          ^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-30T18:18:43.3547250Z error[E0308]: mismatched types
2019-10-30T18:18:43.3548191Z    --> src/tools/clippy/clippy_lints/src/consts.rs:164:32
2019-10-30T18:18:43.3548711Z     |
2019-10-30T18:18:43.3548711Z     |
2019-10-30T18:18:43.3549457Z 164 |         LitKind::Float(ref is, FloatTy::F32) => Constant::F32(is.as_str().parse().unwrap()),
2019-10-30T18:18:43.3550235Z     |                                ^^^^^^^^^^^^ expected enum `syntax::ast::LitFloatType`, found enum `syntax::ast::FloatTy`
2019-10-30T18:18:43.3551178Z     = note: expected type `syntax::ast::LitFloatType`
2019-10-30T18:18:43.3551610Z                found type `syntax::ast::FloatTy`
2019-10-30T18:18:43.3551806Z 
2019-10-30T18:18:43.3558082Z error[E0308]: mismatched types
2019-10-30T18:18:43.3558082Z error[E0308]: mismatched types
2019-10-30T18:18:43.3558633Z    --> src/tools/clippy/clippy_lints/src/consts.rs:165:32
2019-10-30T18:18:43.3559065Z     |
2019-10-30T18:18:43.3559739Z 165 |         LitKind::Float(ref is, FloatTy::F64) => Constant::F64(is.as_str().parse().unwrap()),
2019-10-30T18:18:43.3560353Z     |                                ^^^^^^^^^^^^ expected enum `syntax::ast::LitFloatType`, found enum `syntax::ast::FloatTy`
2019-10-30T18:18:43.3561412Z     = note: expected type `syntax::ast::LitFloatType`
2019-10-30T18:18:43.3561860Z                found type `syntax::ast::FloatTy`
2019-10-30T18:18:43.3562053Z 
2019-10-30T18:18:43.3562053Z 
2019-10-30T18:18:43.4244147Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:43.4245635Z     |
2019-10-30T18:18:43.4245635Z     |
2019-10-30T18:18:43.4246365Z 166 |         LitKind::FloatUnsuffixed(ref is) => match ty.expect("type of float is known").kind {
2019-10-30T18:18:43.4247372Z 
2019-10-30T18:18:43.4247372Z 
2019-10-30T18:18:43.6799746Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:43.6801525Z     |
2019-10-30T18:18:43.6802042Z 292 |                     LitKind::FloatUnsuffixed(_) => {
2019-10-30T18:18:43.6802665Z     |                              ^^^^^^^^^^^^^^^ variant or associated item not found in `syntax::ast::LitKind`
2019-10-30T18:18:43.6802913Z 
2019-10-30T18:18:43.6802913Z 
2019-10-30T18:18:44.3097350Z error[E0308]: mismatched types
2019-10-30T18:18:44.3098343Z   --> src/tools/clippy/clippy_lints/src/approx_const.rs:65:27
2019-10-30T18:18:44.3098905Z    |
2019-10-30T18:18:44.3099473Z 65 |         LitKind::Float(s, FloatTy::F32) => check_known_consts(cx, e, s, "f32"),
2019-10-30T18:18:44.3100811Z    |                           ^^^^^^^^^^^^ expected enum `syntax::ast::LitFloatType`, found enum `syntax::ast::FloatTy`
2019-10-30T18:18:44.3102017Z    = note: expected type `syntax::ast::LitFloatType`
2019-10-30T18:18:44.3102587Z               found type `syntax::ast::FloatTy`
2019-10-30T18:18:44.3102872Z 
2019-10-30T18:18:44.3109320Z error[E0308]: mismatched types
2019-10-30T18:18:44.3109320Z error[E0308]: mismatched types
2019-10-30T18:18:44.3109965Z   --> src/tools/clippy/clippy_lints/src/approx_const.rs:66:27
2019-10-30T18:18:44.3110655Z    |
2019-10-30T18:18:44.3111263Z 66 |         LitKind::Float(s, FloatTy::F64) => check_known_consts(cx, e, s, "f64"),
2019-10-30T18:18:44.3112181Z    |                           ^^^^^^^^^^^^ expected enum `syntax::ast::LitFloatType`, found enum `syntax::ast::FloatTy`
2019-10-30T18:18:44.3113453Z    = note: expected type `syntax::ast::LitFloatType`
2019-10-30T18:18:44.3113986Z               found type `syntax::ast::FloatTy`
2019-10-30T18:18:44.3114239Z 
2019-10-30T18:18:44.3114239Z 
2019-10-30T18:18:44.3144955Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:44.3146198Z    |
2019-10-30T18:18:44.3146198Z    |
2019-10-30T18:18:44.3146813Z 67 |         LitKind::FloatUnsuffixed(s) => check_known_consts(cx, e, s, "f{32, 64}"),
2019-10-30T18:18:44.3147775Z 
2019-10-30T18:18:44.3147775Z 
2019-10-30T18:18:44.8167648Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:44.8169811Z    |
2019-10-30T18:18:44.8169811Z    |
2019-10-30T18:18:44.8170372Z 46 |             if let LitKind::Float(sym, _) | LitKind::FloatUnsuffixed(sym) = lit.node;
2019-10-30T18:18:44.8171301Z 
2019-10-30T18:18:44.8171301Z 
2019-10-30T18:18:45.1902146Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:45.1903541Z     |
2019-10-30T18:18:45.1903975Z 376 |             LitKind::Float(..) | LitKind::FloatUnsuffixed(..) => {
2019-10-30T18:18:45.1904965Z     |                                           ^^^^^^^^^^^^^^^ variant or associated item not found in `syntax::ast::LitKind`
2019-10-30T18:18:45.1905235Z 
2019-10-30T18:18:45.1905235Z 
2019-10-30T18:18:45.9026230Z error[E0599]: no method named `ty_to_string` found for type `syntax::ast::IntTy` in the current scope
2019-10-30T18:18:45.9027574Z    --> src/tools/clippy/clippy_lints/src/misc_early.rs:485:46
2019-10-30T18:18:45.9028012Z     |
2019-10-30T18:18:45.9028461Z 485 |                 LitIntType::Signed(ty) => ty.ty_to_string(),
2019-10-30T18:18:45.9029023Z     |                                              ^^^^^^^^^^^^ help: there is a method with a similar name: `val_to_string`
2019-10-30T18:18:45.9131597Z error[E0599]: no method named `ty_to_string` found for type `syntax::ast::UintTy` in the current scope
2019-10-30T18:18:45.9132951Z    --> src/tools/clippy/clippy_lints/src/misc_early.rs:486:48
2019-10-30T18:18:45.9133534Z     |
2019-10-30T18:18:45.9133534Z     |
2019-10-30T18:18:45.9134005Z 486 |                 LitIntType::Unsigned(ty) => ty.ty_to_string(),
2019-10-30T18:18:45.9134763Z     |                                                ^^^^^^^^^^^^ help: there is a method with a similar name: `val_to_string`
2019-10-30T18:18:45.9134974Z 
2019-10-30T18:18:45.9295523Z error[E0599]: no method named `ty_to_string` found for type `syntax::ast::LitFloatType` in the current scope
2019-10-30T18:18:45.9297015Z     |
2019-10-30T18:18:45.9297015Z     |
2019-10-30T18:18:45.9297488Z 547 |             let suffix = float_ty.ty_to_string();
2019-10-30T18:18:45.9298286Z     |                                   ^^^^^^^^^^^^ method not found in `syntax::ast::LitFloatType`
2019-10-30T18:18:45.9298498Z 
2019-10-30T18:18:46.2563363Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:46.2564058Z    |
2019-10-30T18:18:46.2564417Z 93 | ...                   LitKind::Int(..) | LitKind::Float(..) | LitKind::FloatUnsuffixed(..) => {
2019-10-30T18:18:46.2564897Z    |                                                                        ^^^^^^^^^^^^^^^ variant or associated item not found in `syntax::ast::LitKind`
2019-10-30T18:18:46.2565203Z 
2019-10-30T18:18:46.2565203Z 
2019-10-30T18:18:46.6007576Z error[E0277]: `syntax::ast::UintTy` doesn't implement `std::fmt::Display`
2019-10-30T18:18:46.6008708Z    --> src/tools/clippy/clippy_lints/src/transmute.rs:393:47
2019-10-30T18:18:46.6009288Z     |
2019-10-30T18:18:46.6009851Z 393 | ...                   arg.as_ty(ast::UintTy::U32)
2019-10-30T18:18:46.6010545Z     |                                 ^^^^^^^^^^^^^^^^ `syntax::ast::UintTy` cannot be formatted with the default formatter
2019-10-30T18:18:46.6011880Z     = help: the trait `std::fmt::Display` is not implemented for `syntax::ast::UintTy`
2019-10-30T18:18:46.6012682Z     = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-10-30T18:18:46.6012948Z 
2019-10-30T18:18:46.6012948Z 
2019-10-30T18:18:46.7244295Z error[E0599]: no variant or associated item named `FloatUnsuffixed` found for type `syntax::ast::LitKind` in the current scope
2019-10-30T18:18:46.7245728Z      |
2019-10-30T18:18:46.7245728Z      |
2019-10-30T18:18:46.7246229Z 1189 |                     LitKind::Int(_, LitIntType::Unsuffixed) | LitKind::FloatUnsuffixed(_) => {},
2019-10-30T18:18:46.7247572Z 
2019-10-30T18:18:47.0110903Z error: aborting due to 15 previous errors
2019-10-30T18:18:47.0111031Z 
2019-10-30T18:18:47.0111362Z Some errors have detailed explanations: E0277, E0308, E0599.
---
2019-10-30T18:31:55.3722629Z [RUSTC-TIMING] git2 test:false 22.779
2019-10-30T18:31:56.3916586Z warning: unnecessary parentheses around type
2019-10-30T18:31:56.3917667Z     --> src/tools/rustfmt/src/items.rs:2526:13
2019-10-30T18:31:56.3918163Z      |
2019-10-30T18:31:56.3918715Z 2526 | ) -> Option<((usize, usize, Indent))> {
2019-10-30T18:31:56.3919314Z      |             ^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-30T18:31:56.3920381Z      = note: `#[warn(unused_parens)]` on by default
2019-10-30T18:31:56.3923618Z 
2019-10-30T18:32:15.6881112Z [RUSTC-TIMING] rustc_ap_syntax test:false 182.573
2019-10-30T18:32:15.6912137Z    Compiling git2-curl v0.11.0
---
2019-10-30T18:43:05.3511101Z [RUSTC-TIMING] git_rustfmt test:true 0.494
2019-10-30T18:43:06.7186465Z warning: unnecessary parentheses around type
2019-10-30T18:43:06.7186791Z     --> src/tools/rustfmt/src/items.rs:2526:13
2019-10-30T18:43:06.7187019Z      |
2019-10-30T18:43:06.7187303Z 2526 | ) -> Option<((usize, usize, Indent))> {
2019-10-30T18:43:06.7187829Z      |             ^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-30T18:43:06.7188385Z      = note: `#[warn(unused_parens)]` on by default
2019-10-30T18:43:06.7188438Z 
2019-10-30T18:43:07.4674798Z [RUSTC-TIMING] cargo_fmt test:true 2.611
2019-10-30T18:43:08.1066451Z [RUSTC-TIMING] rustfmt test:true 0.632
---
2019-10-30T18:46:30.9850980Z   local time: Wed Oct 30 18:46:30 UTC 2019
2019-10-30T18:46:30.9991998Z   network time: Wed, 30 Oct 2019 18:46:30 GMT
2019-10-30T18:46:30.9995890Z == end clock drift check ==
2019-10-30T18:46:33.7997681Z 
2019-10-30T18:46:33.8091952Z ##[error]Bash exited with code '1'.
2019-10-30T18:46:33.8122764Z ##[section]Starting: Upload CPU usage statistics
2019-10-30T18:46:33.8128778Z ==============================================================================
2019-10-30T18:46:33.8128863Z Task         : Bash
2019-10-30T18:46:33.8128941Z Description  : Run a Bash script on macOS, Linux, or Windows
