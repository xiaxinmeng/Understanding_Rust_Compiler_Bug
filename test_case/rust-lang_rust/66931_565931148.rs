plain
2019-12-16T06:17:55.3677818Z    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
2019-12-16T06:17:58.4366566Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4367073Z   --> src/tools/clippy/clippy_lints/src/utils/author.rs:67:70
2019-12-16T06:17:58.4367375Z    |
2019-12-16T06:17:58.4368521Z 67 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::Item) {
2019-12-16T06:17:58.4369104Z    |                                                                      ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4369789Z note: lint level defined here
2019-12-16T06:17:58.4370138Z   --> src/tools/clippy/clippy_lints/src/lib.rs:10:9
2019-12-16T06:17:58.4370424Z    |
2019-12-16T06:17:58.4370424Z    |
2019-12-16T06:17:58.4370802Z 10 | #![warn(rust_2018_idioms, trivial_casts, trivial_numeric_casts)]
2019-12-16T06:17:58.4371162Z    |         ^^^^^^^^^^^^^^^^
2019-12-16T06:17:58.4371567Z    = note: `#[warn(elided_lifetimes_in_paths)]` implied by `#[warn(rust_2018_idioms)]`
2019-12-16T06:17:58.4415438Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4416127Z   --> src/tools/clippy/clippy_lints/src/utils/author.rs:76:75
2019-12-16T06:17:58.4416424Z    |
2019-12-16T06:17:58.4416961Z 76 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.4416961Z 76 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.4417617Z    |                                                                           ^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4456184Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4456573Z   --> src/tools/clippy/clippy_lints/src/utils/author.rs:85:76
2019-12-16T06:17:58.4456909Z    |
2019-12-16T06:17:58.4457328Z 85 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::TraitItem) {
2019-12-16T06:17:58.4457328Z 85 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::TraitItem) {
2019-12-16T06:17:58.4457886Z    |                                                                            ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4553683Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4554109Z   --> src/tools/clippy/clippy_lints/src/utils/author.rs:94:72
2019-12-16T06:17:58.4554418Z    |
2019-12-16T06:17:58.4554834Z 94 |     fn check_variant(&mut self, cx: &LateContext<'a, 'tcx>, var: &'tcx hir::Variant) {
2019-12-16T06:17:58.4554834Z 94 |     fn check_variant(&mut self, cx: &LateContext<'a, 'tcx>, var: &'tcx hir::Variant) {
2019-12-16T06:17:58.4555450Z    |                                                                        ^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4566235Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4566689Z    --> src/tools/clippy/clippy_lints/src/utils/author.rs:103:79
2019-12-16T06:17:58.4567024Z     |
2019-12-16T06:17:58.4567024Z     |
2019-12-16T06:17:58.4567472Z 103 |     fn check_struct_field(&mut self, cx: &LateContext<'a, 'tcx>, field: &'tcx hir::StructField) {
2019-12-16T06:17:58.4568069Z     |                                                                               ^^^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4613195Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4613581Z    --> src/tools/clippy/clippy_lints/src/utils/author.rs:139:78
2019-12-16T06:17:58.4613893Z     |
2019-12-16T06:17:58.4614302Z 139 |     fn check_foreign_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::ForeignItem) {
2019-12-16T06:17:58.4614302Z 139 |     fn check_foreign_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::ForeignItem) {
2019-12-16T06:17:58.4614881Z     |                                                                              ^^^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4615324Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4615923Z   --> src/tools/clippy/clippy_lints/src/utils/inspector.rs:37:70
2019-12-16T06:17:58.4616214Z    |
2019-12-16T06:17:58.4616699Z 37 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::Item) {
2019-12-16T06:17:58.4616699Z 37 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::Item) {
2019-12-16T06:17:58.4617285Z    |                                                                      ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4617722Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4618079Z   --> src/tools/clippy/clippy_lints/src/utils/inspector.rs:44:75
2019-12-16T06:17:58.4618348Z    |
2019-12-16T06:17:58.4618774Z 44 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.4618774Z 44 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.4619336Z    |                                                                           ^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4620017Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4620366Z    --> src/tools/clippy/clippy_lints/src/utils/inspector.rs:329:48
2019-12-16T06:17:58.4620658Z     |
2019-12-16T06:17:58.4621030Z 329 | fn print_item(cx: &LateContext<'_, '_>, item: &hir::Item) {
2019-12-16T06:17:58.4621030Z 329 | fn print_item(cx: &LateContext<'_, '_>, item: &hir::Item) {
2019-12-16T06:17:58.4621522Z     |                                                ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4621969Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4622338Z    --> src/tools/clippy/clippy_lints/src/utils/internal_lints.rs:168:70
2019-12-16T06:17:58.4622621Z     |
2019-12-16T06:17:58.4622621Z     |
2019-12-16T06:17:58.4622992Z 168 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx Item) {
2019-12-16T06:17:58.4623525Z     |                                                                      ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4623951Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4624313Z    --> src/tools/clippy/clippy_lints/src/utils/internal_lints.rs:194:73
2019-12-16T06:17:58.4624593Z     |
2019-12-16T06:17:58.4624593Z     |
2019-12-16T06:17:58.4624992Z 194 |     fn check_crate_post(&mut self, cx: &LateContext<'a, 'tcx>, _: &'tcx Crate) {
2019-12-16T06:17:58.4625523Z     |                                                                         ^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4625947Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4626295Z   --> src/tools/clippy/clippy_lints/src/utils/ptr.rs:29:17
2019-12-16T06:17:58.4626558Z    |
2019-12-16T06:17:58.4627030Z 29 |     body: &'tcx Body,
2019-12-16T06:17:58.4627030Z 29 |     body: &'tcx Body,
2019-12-16T06:17:58.4627580Z    |                 ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4628014Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4628368Z    --> src/tools/clippy/clippy_lints/src/utils/mod.rs:904:57
2019-12-16T06:17:58.4628640Z     |
2019-12-16T06:17:58.4628640Z     |
2019-12-16T06:17:58.4629043Z 904 | pub fn iter_input_pats<'tcx>(decl: &FnDecl, body: &'tcx Body) -> impl Iterator<Item = &'tcx Param> {
2019-12-16T06:17:58.4629571Z     |                                                         ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4630004Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4630349Z    --> src/tools/clippy/clippy_lints/src/arithmetic.rs:116:63
2019-12-16T06:17:58.4630632Z     |
2019-12-16T06:17:58.4631019Z 116 |     fn check_body(&mut self, cx: &LateContext<'_, '_>, body: &hir::Body) {
2019-12-16T06:17:58.4631019Z 116 |     fn check_body(&mut self, cx: &LateContext<'_, '_>, body: &hir::Body) {
2019-12-16T06:17:58.4631534Z     |                                                               ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4631956Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4632320Z    --> src/tools/clippy/clippy_lints/src/arithmetic.rs:134:68
2019-12-16T06:17:58.4632579Z     |
2019-12-16T06:17:58.4632579Z     |
2019-12-16T06:17:58.4632979Z 134 |     fn check_body_post(&mut self, cx: &LateContext<'_, '_>, body: &hir::Body) {
2019-12-16T06:17:58.4633522Z     |                                                                    ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4633947Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4634278Z    --> src/tools/clippy/clippy_lints/src/attrs.rs:227:70
2019-12-16T06:17:58.4634556Z     |
2019-12-16T06:17:58.4634556Z     |
2019-12-16T06:17:58.4634926Z 227 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx Item) {
2019-12-16T06:17:58.4635481Z     |                                                                      ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4635905Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4636248Z    --> src/tools/clippy/clippy_lints/src/attrs.rs:298:75
2019-12-16T06:17:58.4636509Z     |
2019-12-16T06:17:58.4636908Z 298 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx ImplItem) {
2019-12-16T06:17:58.4636908Z 298 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx ImplItem) {
2019-12-16T06:17:58.4637446Z     |                                                                           ^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4638113Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4638538Z    --> src/tools/clippy/clippy_lints/src/attrs.rs:304:76
2019-12-16T06:17:58.4638845Z     |
2019-12-16T06:17:58.4639265Z 304 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx TraitItem) {
2019-12-16T06:17:58.4639265Z 304 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx TraitItem) {
2019-12-16T06:17:58.4639852Z     |                                                                            ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4640311Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4640684Z    --> src/tools/clippy/clippy_lints/src/attrs.rs:358:54
2019-12-16T06:17:58.4640983Z     |
2019-12-16T06:17:58.4641400Z 358 | fn is_relevant_item(cx: &LateContext<'_, '_>, item: &Item) -> bool {
2019-12-16T06:17:58.4641400Z 358 | fn is_relevant_item(cx: &LateContext<'_, '_>, item: &Item) -> bool {
2019-12-16T06:17:58.4641937Z     |                                                      ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4642386Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4642763Z    --> src/tools/clippy/clippy_lints/src/attrs.rs:366:54
2019-12-16T06:17:58.4643047Z     |
2019-12-16T06:17:58.4643581Z 366 | fn is_relevant_impl(cx: &LateContext<'_, '_>, item: &ImplItem) -> bool {
2019-12-16T06:17:58.4643581Z 366 | fn is_relevant_impl(cx: &LateContext<'_, '_>, item: &ImplItem) -> bool {
2019-12-16T06:17:58.4644101Z     |                                                      ^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4684280Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4684667Z    --> src/tools/clippy/clippy_lints/src/attrs.rs:373:55
2019-12-16T06:17:58.4684956Z     |
2019-12-16T06:17:58.4684956Z     |
2019-12-16T06:17:58.4685336Z 373 | fn is_relevant_trait(cx: &LateContext<'_, '_>, item: &TraitItem) -> bool {
2019-12-16T06:17:58.4685863Z     |                                                       ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4707845Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4708235Z   --> src/tools/clippy/clippy_lints/src/booleans.rs:64:21
2019-12-16T06:17:58.4708503Z    |
2019-12-16T06:17:58.4708823Z 64 |         body: &'tcx Body,
2019-12-16T06:17:58.4708823Z 64 |         body: &'tcx Body,
2019-12-16T06:17:58.4709254Z    |                     ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4745508Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4745929Z   --> src/tools/clippy/clippy_lints/src/cognitive_complexity.rs:44:78
2019-12-16T06:17:58.4746537Z    |
2019-12-16T06:17:58.4746537Z    |
2019-12-16T06:17:58.4747019Z 44 |     fn check<'a, 'tcx>(&mut self, cx: &'a LateContext<'a, 'tcx>, body: &'tcx Body, span: Span) {
2019-12-16T06:17:58.4747655Z    |                                                                              ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4784491Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4784907Z   --> src/tools/clippy/clippy_lints/src/cognitive_complexity.rs:89:21
2019-12-16T06:17:58.4785188Z    |
2019-12-16T06:17:58.4785514Z 89 |         body: &'tcx Body,
2019-12-16T06:17:58.4785514Z 89 |         body: &'tcx Body,
2019-12-16T06:17:58.4785979Z    |                     ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4864240Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4864663Z   --> src/tools/clippy/clippy_lints/src/copy_iterator.rs:36:70
2019-12-16T06:17:58.4864953Z    |
2019-12-16T06:17:58.4864953Z    |
2019-12-16T06:17:58.4865336Z 36 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx Item) {
2019-12-16T06:17:58.4865872Z    |                                                                      ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4874421Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4874797Z   --> src/tools/clippy/clippy_lints/src/derive.rs:69:70
2019-12-16T06:17:58.4875102Z    |
2019-12-16T06:17:58.4875102Z    |
2019-12-16T06:17:58.4875496Z 69 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx Item) {
2019-12-16T06:17:58.4876018Z    |                                                                      ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4901295Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4901670Z    --> src/tools/clippy/clippy_lints/src/derive.rs:133:66
2019-12-16T06:17:58.4901976Z     |
2019-12-16T06:17:58.4901976Z     |
2019-12-16T06:17:58.4902399Z 133 | fn check_copy_clone<'a, 'tcx>(cx: &LateContext<'a, 'tcx>, item: &Item, trait_ref: &TraitRef, ty: Ty<'tcx>) {
2019-12-16T06:17:58.4902947Z     |                                                                  ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4903372Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4903700Z    --> src/tools/clippy/clippy_lints/src/doc.rs:149:72
2019-12-16T06:17:58.4903988Z     |
2019-12-16T06:17:58.4903988Z     |
2019-12-16T06:17:58.4904388Z 149 |     fn check_crate(&mut self, cx: &LateContext<'a, 'tcx>, krate: &'tcx hir::Crate) {
2019-12-16T06:17:58.4905207Z     |                                                                        ^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4905680Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4906029Z    --> src/tools/clippy/clippy_lints/src/doc.rs:153:70
2019-12-16T06:17:58.4906290Z     |
2019-12-16T06:17:58.4906688Z 153 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::Item) {
2019-12-16T06:17:58.4906688Z 153 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::Item) {
2019-12-16T06:17:58.4907227Z     |                                                                      ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4907667Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4908004Z    --> src/tools/clippy/clippy_lints/src/doc.rs:166:76
2019-12-16T06:17:58.4908284Z     |
2019-12-16T06:17:58.4908284Z     |
2019-12-16T06:17:58.4908671Z 166 |     fn check_item_post(&mut self, _cx: &LateContext<'a, 'tcx>, item: &'tcx hir::Item) {
2019-12-16T06:17:58.4909691Z     |                                                                            ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4910199Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4910666Z    --> src/tools/clippy/clippy_lints/src/doc.rs:172:76
2019-12-16T06:17:58.4910919Z     |
2019-12-16T06:17:58.4911329Z 172 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::TraitItem) {
2019-12-16T06:17:58.4911329Z 172 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::TraitItem) {
2019-12-16T06:17:58.4911900Z     |                                                                            ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4912351Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4912720Z    --> src/tools/clippy/clippy_lints/src/doc.rs:179:75
2019-12-16T06:17:58.4913012Z     |
2019-12-16T06:17:58.4913420Z 179 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.4913420Z 179 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.4914012Z     |                                                                           ^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4914455Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4914806Z   --> src/tools/clippy/clippy_lints/src/empty_enum.rs:30:63
2019-12-16T06:17:58.4915098Z    |
2019-12-16T06:17:58.4915490Z 30 |     fn check_item(&mut self, cx: &LateContext<'_, '_>, item: &Item) {
2019-12-16T06:17:58.4915490Z 30 |     fn check_item(&mut self, cx: &LateContext<'_, '_>, item: &Item) {
2019-12-16T06:17:58.4916015Z    |                                                               ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4916688Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4917097Z   --> src/tools/clippy/clippy_lints/src/enum_clike.rs:44:70
2019-12-16T06:17:58.4917379Z    |
2019-12-16T06:17:58.4917379Z    |
2019-12-16T06:17:58.4917783Z 44 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx Item) {
2019-12-16T06:17:58.4918324Z    |                                                                      ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4948241Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4948649Z   --> src/tools/clippy/clippy_lints/src/enum_glob_use.rs:32:66
2019-12-16T06:17:58.4948943Z    |
2019-12-16T06:17:58.4948943Z    |
2019-12-16T06:17:58.4949410Z 32 |     fn check_mod(&mut self, cx: &LateContext<'a, 'tcx>, m: &'tcx Mod, _: Span, _: HirId) {
2019-12-16T06:17:58.4950002Z    |                                                                  ^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.4969211Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.4969604Z   --> src/tools/clippy/clippy_lints/src/enum_glob_use.rs:41:47
2019-12-16T06:17:58.4969870Z    |
2019-12-16T06:17:58.4970242Z 41 | fn lint_item(cx: &LateContext<'_, '_>, item: &Item) {
2019-12-16T06:17:58.4970242Z 41 | fn lint_item(cx: &LateContext<'_, '_>, item: &Item) {
2019-12-16T06:17:58.4970734Z    |                                               ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5006900Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5007283Z   --> src/tools/clippy/clippy_lints/src/escape.rs:59:21
2019-12-16T06:17:58.5007574Z    |
2019-12-16T06:17:58.5007886Z 59 |         body: &'tcx Body,
2019-12-16T06:17:58.5007886Z 59 |         body: &'tcx Body,
2019-12-16T06:17:58.5008349Z    |                     ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5043952Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5044486Z   --> src/tools/clippy/clippy_lints/src/fallible_impl_from.rs:35:70
2019-12-16T06:17:58.5044775Z    |
2019-12-16T06:17:58.5045173Z 35 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::Item) {
2019-12-16T06:17:58.5045173Z 35 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::Item) {
2019-12-16T06:17:58.5045700Z    |                                                                      ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5144594Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5145123Z    --> src/tools/clippy/clippy_lints/src/functions.rs:190:21
2019-12-16T06:17:58.5145631Z     |
2019-12-16T06:17:58.5145952Z 190 |         body: &'tcx hir::Body,
2019-12-16T06:17:58.5145952Z 190 |         body: &'tcx hir::Body,
2019-12-16T06:17:58.5146511Z     |                     ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5155377Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5155772Z    --> src/tools/clippy/clippy_lints/src/functions.rs:230:70
2019-12-16T06:17:58.5156046Z     |
2019-12-16T06:17:58.5156443Z 230 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::Item) {
2019-12-16T06:17:58.5156443Z 230 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::Item) {
2019-12-16T06:17:58.5156997Z     |                                                                      ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5165949Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5166527Z    --> src/tools/clippy/clippy_lints/src/functions.rs:252:75
2019-12-16T06:17:58.5167866Z     |
2019-12-16T06:17:58.5168370Z 252 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.5168370Z 252 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.5169008Z     |                                                                           ^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5170734Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5172665Z    --> src/tools/clippy/clippy_lints/src/functions.rs:275:76
2019-12-16T06:17:58.5178639Z     |
2019-12-16T06:17:58.5186598Z 275 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::TraitItem) {
2019-12-16T06:17:58.5186598Z 275 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::TraitItem) {
2019-12-16T06:17:58.5197303Z     |                                                                            ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5197963Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5198368Z    --> src/tools/clippy/clippy_lints/src/functions.rs:320:82
2019-12-16T06:17:58.5198705Z     |
2019-12-16T06:17:58.5199185Z 320 |     fn check_line_number(self, cx: &LateContext<'_, '_>, span: Span, body: &'tcx hir::Body) {
2019-12-16T06:17:58.5199185Z 320 |     fn check_line_number(self, cx: &LateContext<'_, '_>, span: Span, body: &'tcx hir::Body) {
2019-12-16T06:17:58.5199820Z     |                                                                                  ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5200310Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5201175Z    --> src/tools/clippy/clippy_lints/src/functions.rs:378:21
2019-12-16T06:17:58.5201526Z     |
2019-12-16T06:17:58.5201907Z 378 |         body: &'tcx hir::Body,
2019-12-16T06:17:58.5201907Z 378 |         body: &'tcx hir::Body,
2019-12-16T06:17:58.5202815Z     |                     ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5203335Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5203733Z    --> src/tools/clippy/clippy_lints/src/functions.rs:442:17
2019-12-16T06:17:58.5204045Z     |
2019-12-16T06:17:58.5204398Z 442 |     body: &'tcx hir::Body,
2019-12-16T06:17:58.5204398Z 442 |     body: &'tcx hir::Body,
2019-12-16T06:17:58.5204911Z     |                 ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5205383Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5205759Z    --> src/tools/clippy/clippy_lints/src/functions.rs:524:53
2019-12-16T06:17:58.5206081Z     |
2019-12-16T06:17:58.5206499Z 524 | fn has_mutable_arg(cx: &LateContext<'_, '_>, body: &hir::Body) -> bool {
2019-12-16T06:17:58.5206499Z 524 | fn has_mutable_arg(cx: &LateContext<'_, '_>, body: &hir::Body) -> bool {
2019-12-16T06:17:58.5208330Z     |                                                     ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5208888Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5209283Z    --> src/tools/clippy/clippy_lints/src/functions.rs:689:72
2019-12-16T06:17:58.5209577Z     |
2019-12-16T06:17:58.5209577Z     |
2019-12-16T06:17:58.5210053Z 689 | fn mutates_static<'a, 'tcx>(cx: &'a LateContext<'a, 'tcx>, body: &'tcx hir::Body) -> bool {
2019-12-16T06:17:58.5211704Z     |                                                                        ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5212261Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5212688Z    --> src/tools/clippy/clippy_lints/src/implicit_return.rs:133:21
2019-12-16T06:17:58.5213046Z     |
2019-12-16T06:17:58.5213435Z 133 |         body: &'tcx Body,
2019-12-16T06:17:58.5213435Z 133 |         body: &'tcx Body,
2019-12-16T06:17:58.5213987Z     |                     ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5214506Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5214940Z   --> src/tools/clippy/clippy_lints/src/inherent_impl.rs:52:69
2019-12-16T06:17:58.5215262Z    |
2019-12-16T06:17:58.5215262Z    |
2019-12-16T06:17:58.5215726Z 52 |     fn check_item(&mut self, _: &LateContext<'a, 'tcx>, item: &'tcx Item) {
2019-12-16T06:17:58.5216348Z    |                                                                     ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5216860Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5217476Z   --> src/tools/clippy/clippy_lints/src/inherent_impl.rs:63:77
2019-12-16T06:17:58.5217827Z    |
2019-12-16T06:17:58.5217827Z    |
2019-12-16T06:17:58.5218405Z 63 |     fn check_crate_post(&mut self, cx: &LateContext<'a, 'tcx>, krate: &'tcx Crate) {
2019-12-16T06:17:58.5219082Z    |                                                                             ^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5220186Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5220698Z   --> src/tools/clippy/clippy_lints/src/inherent_to_string.rs:97:80
2019-12-16T06:17:58.5221028Z    |
2019-12-16T06:17:58.5221028Z    |
2019-12-16T06:17:58.5221494Z 97 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, impl_item: &'tcx ImplItem) {
2019-12-16T06:17:58.5222161Z    |                                                                                ^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5222684Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5223124Z    --> src/tools/clippy/clippy_lints/src/inherent_to_string.rs:123:47
2019-12-16T06:17:58.5223453Z     |
2019-12-16T06:17:58.5223901Z 123 | fn show_lint(cx: &LateContext<'_, '_>, item: &ImplItem) {
2019-12-16T06:17:58.5223901Z 123 | fn show_lint(cx: &LateContext<'_, '_>, item: &ImplItem) {
2019-12-16T06:17:58.5224501Z     |                                               ^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5225032Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5225460Z   --> src/tools/clippy/clippy_lints/src/inline_fn_without_body.rs:35:76
2019-12-16T06:17:58.5225802Z    |
2019-12-16T06:17:58.5226257Z 35 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx TraitItem) {
2019-12-16T06:17:58.5226257Z 35 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx TraitItem) {
2019-12-16T06:17:58.5226900Z    |                                                                            ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5228150Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5228856Z   --> src/tools/clippy/clippy_lints/src/large_enum_variant.rs:50:63
2019-12-16T06:17:58.5229184Z    |
2019-12-16T06:17:58.5229616Z 50 |     fn check_item(&mut self, cx: &LateContext<'_, '_>, item: &Item) {
2019-12-16T06:17:58.5229616Z 50 |     fn check_item(&mut self, cx: &LateContext<'_, '_>, item: &Item) {
2019-12-16T06:17:58.5230518Z    |                                                               ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5231067Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5231451Z   --> src/tools/clippy/clippy_lints/src/len_zero.rs:76:70
2019-12-16T06:17:58.5231987Z    |
2019-12-16T06:17:58.5231987Z    |
2019-12-16T06:17:58.5232408Z 76 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx Item) {
2019-12-16T06:17:58.5233104Z    |                                                                      ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5262294Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5262822Z    --> src/tools/clippy/clippy_lints/src/len_zero.rs:119:64
2019-12-16T06:17:58.5263139Z     |
2019-12-16T06:17:58.5263139Z     |
2019-12-16T06:17:58.5263614Z 119 | fn check_trait_items(cx: &LateContext<'_, '_>, visited_trait: &Item, trait_items: &[TraitItemRef]) {
2019-12-16T06:17:58.5264229Z     |                                                                ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5277684Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5280859Z    --> src/tools/clippy/clippy_lints/src/len_zero.rs:170:54
2019-12-16T06:17:58.5281257Z     |
2019-12-16T06:17:58.5281257Z     |
2019-12-16T06:17:58.5281710Z 170 | fn check_impl_items(cx: &LateContext<'_, '_>, item: &Item, impl_items: &[ImplItemRef]) {
2019-12-16T06:17:58.5282308Z     |                                                      ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5346164Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5346813Z   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:74:70
2019-12-16T06:17:58.5347184Z    |
2019-12-16T06:17:58.5347184Z    |
2019-12-16T06:17:58.5347671Z 74 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx Item) {
2019-12-16T06:17:58.5348289Z    |                                                                      ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5349969Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5352857Z   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:80:75
2019-12-16T06:17:58.5353282Z    |
2019-12-16T06:17:58.5353749Z 80 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx ImplItem) {
2019-12-16T06:17:58.5353749Z 80 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx ImplItem) {
2019-12-16T06:17:58.5356600Z    |                                                                           ^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5358083Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5358540Z   --> src/tools/clippy/clippy_lints/src/lifetimes.rs:94:76
2019-12-16T06:17:58.5358860Z    |
2019-12-16T06:17:58.5360508Z 94 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx TraitItem) {
2019-12-16T06:17:58.5360508Z 94 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx TraitItem) {
2019-12-16T06:17:58.5361559Z    |                                                                            ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5363160Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5363606Z   --> src/tools/clippy/clippy_lints/src/main_recursion.rs:38:64
2019-12-16T06:17:58.5363950Z    |
2019-12-16T06:17:58.5363950Z    |
2019-12-16T06:17:58.5365654Z 38 |     fn check_crate(&mut self, _: &LateContext<'_, '_>, krate: &Crate) {
2019-12-16T06:17:58.5366369Z    |                                                                ^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5422224Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5422852Z     --> src/tools/clippy/clippy_lints/src/methods/mod.rs:1238:80
2019-12-16T06:17:58.5423219Z      |
2019-12-16T06:17:58.5423724Z 1238 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, impl_item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.5423724Z 1238 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, impl_item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.5424384Z      |                                                                                ^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5426231Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5426748Z    --> src/tools/clippy/clippy_lints/src/misc.rs:241:21
2019-12-16T06:17:58.5427089Z     |
2019-12-16T06:17:58.5428424Z 241 |         body: &'tcx Body,
2019-12-16T06:17:58.5428424Z 241 |         body: &'tcx Body,
2019-12-16T06:17:58.5429032Z     |                     ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5429523Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5429939Z   --> src/tools/clippy/clippy_lints/src/missing_const_for_fn.rs:80:13
2019-12-16T06:17:58.5430280Z    |
2019-12-16T06:17:58.5430670Z 80 |         _: &Body,
2019-12-16T06:17:58.5430670Z 80 |         _: &Body,
2019-12-16T06:17:58.5431188Z    |             ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5431696Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5432124Z    --> src/tools/clippy/clippy_lints/src/missing_doc.rs:128:72
2019-12-16T06:17:58.5432448Z     |
2019-12-16T06:17:58.5432448Z     |
2019-12-16T06:17:58.5432928Z 128 |     fn check_crate(&mut self, cx: &LateContext<'a, 'tcx>, krate: &'tcx hir::Crate) {
2019-12-16T06:17:58.5433572Z     |                                                                        ^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5434062Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5434737Z    --> src/tools/clippy/clippy_lints/src/missing_doc.rs:132:68
2019-12-16T06:17:58.5435080Z     |
2019-12-16T06:17:58.5435642Z 132 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, it: &'tcx hir::Item) {
2019-12-16T06:17:58.5435642Z 132 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, it: &'tcx hir::Item) {
2019-12-16T06:17:58.5436326Z     |                                                                    ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5436832Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5437242Z    --> src/tools/clippy/clippy_lints/src/missing_doc.rs:165:82
2019-12-16T06:17:58.5437585Z     |
2019-12-16T06:17:58.5438095Z 165 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, trait_item: &'tcx hir::TraitItem) {
2019-12-16T06:17:58.5438095Z 165 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, trait_item: &'tcx hir::TraitItem) {
2019-12-16T06:17:58.5438755Z     |                                                                                  ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5439325Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5439759Z    --> src/tools/clippy/clippy_lints/src/missing_doc.rs:175:80
2019-12-16T06:17:58.5440096Z     |
2019-12-16T06:17:58.5440576Z 175 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, impl_item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.5440576Z 175 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, impl_item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.5441262Z     |                                                                                ^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5467465Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5468061Z    --> src/tools/clippy/clippy_lints/src/missing_doc.rs:196:76
2019-12-16T06:17:58.5468414Z     |
2019-12-16T06:17:58.5468414Z     |
2019-12-16T06:17:58.5468879Z 196 |     fn check_struct_field(&mut self, cx: &LateContext<'a, 'tcx>, sf: &'tcx hir::StructField) {
2019-12-16T06:17:58.5469499Z     |                                                                            ^^^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5546615Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5547278Z    --> src/tools/clippy/clippy_lints/src/missing_doc.rs:202:70
2019-12-16T06:17:58.5547621Z     |
2019-12-16T06:17:58.5548063Z 202 |     fn check_variant(&mut self, cx: &LateContext<'a, 'tcx>, v: &'tcx hir::Variant) {
2019-12-16T06:17:58.5548063Z 202 |     fn check_variant(&mut self, cx: &LateContext<'a, 'tcx>, v: &'tcx hir::Variant) {
2019-12-16T06:17:58.5551936Z     |                                                                      ^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5552507Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5553155Z   --> src/tools/clippy/clippy_lints/src/missing_inline.rs:84:68
2019-12-16T06:17:58.5554619Z    |
2019-12-16T06:17:58.5555262Z 84 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, it: &'tcx hir::Item) {
2019-12-16T06:17:58.5555262Z 84 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, it: &'tcx hir::Item) {
2019-12-16T06:17:58.5556821Z    |                                                                    ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5557411Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5557822Z    --> src/tools/clippy/clippy_lints/src/missing_inline.rs:133:80
2019-12-16T06:17:58.5558114Z     |
2019-12-16T06:17:58.5560373Z 133 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, impl_item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.5560373Z 133 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, impl_item: &'tcx hir::ImplItem) {
2019-12-16T06:17:58.5562030Z     |                                                                                ^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5562608Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5562992Z    --> src/tools/clippy/clippy_lints/src/needless_borrow.rs:111:69
2019-12-16T06:17:58.5564175Z     |
2019-12-16T06:17:58.5564175Z     |
2019-12-16T06:17:58.5564706Z 111 |     fn check_item(&mut self, _: &LateContext<'a, 'tcx>, item: &'tcx Item) {
2019-12-16T06:17:58.5565290Z     |                                                                     ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5566857Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5567272Z    --> src/tools/clippy/clippy_lints/src/needless_borrow.rs:118:74
2019-12-16T06:17:58.5567565Z     |
2019-12-16T06:17:58.5567565Z     |
2019-12-16T06:17:58.5568885Z 118 |     fn check_item_post(&mut self, _: &LateContext<'a, 'tcx>, item: &'tcx Item) {
2019-12-16T06:17:58.5569571Z     |                                                                          ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5577112Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5577673Z   --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:75:21
2019-12-16T06:17:58.5578029Z    |
2019-12-16T06:17:58.5578401Z 75 |         body: &'tcx Body,
2019-12-16T06:17:58.5578401Z 75 |         body: &'tcx Body,
2019-12-16T06:17:58.5578900Z    |                     ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5579355Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5588150Z   --> src/tools/clippy/clippy_lints/src/new_without_default.rs:96:70
2019-12-16T06:17:58.5588530Z    |
2019-12-16T06:17:58.5588981Z 96 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::Item) {
2019-12-16T06:17:58.5588981Z 96 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx hir::Item) {
2019-12-16T06:17:58.5589913Z    |                                                                      ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5590437Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5590826Z    --> src/tools/clippy/clippy_lints/src/non_copy_const.rs:146:68
2019-12-16T06:17:58.5591138Z     |
2019-12-16T06:17:58.5591138Z     |
2019-12-16T06:17:58.5591556Z 146 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, it: &'tcx Item) {
2019-12-16T06:17:58.5592146Z     |                                                                    ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5592645Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5593045Z    --> src/tools/clippy/clippy_lints/src/non_copy_const.rs:153:82
2019-12-16T06:17:58.5593342Z     |
2019-12-16T06:17:58.5593797Z 153 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, trait_item: &'tcx TraitItem) {
2019-12-16T06:17:58.5593797Z 153 |     fn check_trait_item(&mut self, cx: &LateContext<'a, 'tcx>, trait_item: &'tcx TraitItem) {
2019-12-16T06:17:58.5594405Z     |                                                                                  ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5594896Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5595296Z    --> src/tools/clippy/clippy_lints/src/non_copy_const.rs:167:80
2019-12-16T06:17:58.5595617Z     |
2019-12-16T06:17:58.5595617Z     |
2019-12-16T06:17:58.5596107Z 167 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, impl_item: &'tcx ImplItem) {
2019-12-16T06:17:58.5596801Z     |                                                                                ^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5597322Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5597740Z   --> src/tools/clippy/clippy_lints/src/partialeq_ne_impl.rs:35:70
2019-12-16T06:17:58.5598098Z    |
2019-12-16T06:17:58.5598098Z    |
2019-12-16T06:17:58.5598568Z 35 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx Item) {
2019-12-16T06:17:58.5599190Z    |                                                                      ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5599775Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5600193Z    --> src/tools/clippy/clippy_lints/src/ptr.rs:104:70
2019-12-16T06:17:58.5601179Z     |
2019-12-16T06:17:58.5601179Z     |
2019-12-16T06:17:58.5602024Z 104 |     fn check_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx Item) {
2019-12-16T06:17:58.5602839Z     |                                                                      ^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5603444Z warning: hidden lifetime parameters in types are deprecated
2019-12-16T06:17:58.5603814Z    --> src/tools/clippy/clippy_lints/src/ptr.rs:110:75
2019-12-16T06:17:58.5604126Z     |
2019-12-16T06:17:58.5604554Z 110 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx ImplItem) {
2019-12-16T06:17:58.5604554Z 110 |     fn check_impl_item(&mut self, cx: &LateContext<'a, 'tcx>, item: &'tcx ImplItem) {
2019-12-16T06:17:58.5605185Z     |                                                                           ^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-16T06:17:58.5605675Z warning: hidden lifetime parameters in types are deprecated
---
2019-12-16T06:58:31.7042784Z +For more information about this error, try `rustc --explain E0658`.
2019-12-16T06:58:31.7043605Z +
2019-12-16T06:58:31.7043654Z 
2019-12-16T06:58:31.7043725Z The actual stderr differed from the expected stderr.
2019-12-16T06:58:31.7044143Z Actual stderr saved to /tmp/compiletestt3g6LN/async-fn.stderr
2019-12-16T06:58:31.7044258Z To update references, run this command from build directory:
2019-12-16T06:58:31.7044564Z tests/run-pass/update-references.sh '/tmp/compiletestt3g6LN' 'async-fn.rs'
2019-12-16T06:58:31.7044708Z error: 1 errors occurred comparing output.
2019-12-16T06:58:31.7045810Z status: exit code: 1
2019-12-16T06:58:31.7045810Z status: exit code: 1
2019-12-16T06:58:31.7047837Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestt3g6LN" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestt3g6LN/async-fn.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestt3g6LN/async-fn.stage-id.aux" "-A" "unused"
2019-12-16T06:58:31.7048403Z ------------------------------------------
2019-12-16T06:58:31.7048458Z 
2019-12-16T06:58:31.7048727Z ------------------------------------------
2019-12-16T06:58:31.7048803Z stderr:
---
2019-12-16T06:58:37.8560054Z +For more information about this error, try `rustc --explain E0658`.
2019-12-16T06:58:37.8560141Z +
2019-12-16T06:58:37.8560177Z 
2019-12-16T06:58:37.8560269Z The actual stderr differed from the expected stderr.
2019-12-16T06:58:37.8560360Z Actual stderr saved to /tmp/compiletestt3g6LN/generator.stderr
2019-12-16T06:58:37.8560691Z To update references, run this command from build directory:
2019-12-16T06:58:37.8561082Z tests/run-pass/update-references.sh '/tmp/compiletestt3g6LN' 'generator.rs'
2019-12-16T06:58:37.8561343Z error: 1 errors occurred comparing output.
2019-12-16T06:58:37.8561430Z status: exit code: 1
2019-12-16T06:58:37.8561430Z status: exit code: 1
2019-12-16T06:58:37.8562317Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestt3g6LN" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestt3g6LN/generator.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestt3g6LN/generator.stage-id.aux" "-A" "unused"
2019-12-16T06:58:37.8562840Z ------------------------------------------
2019-12-16T06:58:37.8562913Z 
2019-12-16T06:58:37.8563196Z ------------------------------------------
2019-12-16T06:58:37.8563293Z stderr:
---
2019-12-16T06:58:43.5919588Z +For more information about this error, try `rustc --explain E0658`.
2019-12-16T06:58:43.5919674Z +
2019-12-16T06:58:43.5919711Z 
2019-12-16T06:58:43.5919800Z The actual stderr differed from the expected stderr.
2019-12-16T06:58:43.5920130Z Actual stderr saved to /tmp/compiletestt3g6LN/loop-break-value.stderr
2019-12-16T06:58:43.5920443Z To update references, run this command from build directory:
2019-12-16T06:58:43.5920842Z tests/run-pass/update-references.sh '/tmp/compiletestt3g6LN' 'loop-break-value.rs'
2019-12-16T06:58:43.5921097Z error: 1 errors occurred comparing output.
2019-12-16T06:58:43.5921185Z status: exit code: 1
2019-12-16T06:58:43.5921185Z status: exit code: 1
2019-12-16T06:58:43.5922339Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletestt3g6LN" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestt3g6LN/loop-break-value.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestt3g6LN/loop-break-value.stage-id.aux" "-A" "unused"
2019-12-16T06:58:43.5922843Z ------------------------------------------
2019-12-16T06:58:43.5922922Z 
2019-12-16T06:58:43.5923173Z ------------------------------------------
2019-12-16T06:58:43.5923264Z stderr:
---
2019-12-16T06:58:45.5767829Z +For more information about this error, try `rustc --explain E0658`.
2019-12-16T06:58:45.5767932Z  
2019-12-16T06:58:45.5767970Z 
2019-12-16T06:58:45.5768060Z The actual stderr differed from the expected stderr.
2019-12-16T06:58:45.5768155Z Actual stderr saved to /tmp/compiletestt3g6LN/panic/catch_panic.stderr
2019-12-16T06:58:45.5768267Z To update references, run this command from build directory:
2019-12-16T06:58:45.5768597Z tests/run-pass/update-references.sh '/tmp/compiletestt3g6LN' 'panic/catch_panic.rs'
2019-12-16T06:58:45.5768748Z error: 1 errors occurred comparing output.
2019-12-16T06:58:45.5768838Z status: exit code: 1
2019-12-16T06:58:45.5768838Z status: exit code: 1
2019-12-16T06:58:45.5769884Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/panic/catch_panic.rs" "-L" "/tmp/compiletestt3g6LN" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestt3g6LN/panic/catch_panic.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestt3g6LN/panic/catch_panic.stage-id.aux" "-A" "unused"
2019-12-16T06:58:45.5770536Z ------------------------------------------
2019-12-16T06:58:45.5770594Z 
2019-12-16T06:58:45.5770876Z ------------------------------------------
2019-12-16T06:58:45.5770951Z stderr:
---
2019-12-16T06:58:55.9192330Z Verifying status of clippy-driver...
2019-12-16T06:58:55.9192414Z Verifying status of miri...
2019-12-16T06:58:55.9192698Z Verifying status of embedded-book...
2019-12-16T06:58:55.9192996Z Verifying status of rustc-guide...
2019-12-16T06:58:55.9193339Z error: Tool `clippy-driver` should be test-pass but is build-fail during beta week.
2019-12-16T06:58:55.9197861Z Build completed unsuccessfully in 0:00:02
2019-12-16T06:58:55.9257130Z == clock drift check ==
2019-12-16T06:58:55.9271825Z   local time: Mon Dec 16 06:58:55 UTC 2019
2019-12-16T06:58:56.0790306Z   network time: Mon, 16 Dec 2019 06:58:56 GMT
2019-12-16T06:58:56.0790306Z   network time: Mon, 16 Dec 2019 06:58:56 GMT
2019-12-16T06:58:56.0790599Z == end clock drift check ==
2019-12-16T06:58:56.9493172Z 
2019-12-16T06:58:56.9605282Z ##[error]Bash exited with code '1'.
2019-12-16T06:58:56.9651487Z ##[section]Starting: Checkout
2019-12-16T06:58:56.9653952Z ==============================================================================
2019-12-16T06:58:56.9654079Z Task         : Get sources
2019-12-16T06:58:56.9654174Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
