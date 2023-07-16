plain
    Checking toml v0.5.7
    Checking cargo_metadata v0.15.3
    Checking rustfix v0.6.1
    Checking clippy_lints v0.1.70 (/checkout/src/tools/clippy/clippy_lints)
error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::PathSegment<'_>>`
    |
    |
429 |                 if let [.., name] = path.segments;
    |                        ^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::PathSegment<'_>>`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:407:28
    |
    |
407 |                 self.slice(args, |e| self.expr(e));
    |                      ----- ^^^^ expected `&Binding<&[Expr<'_>]>`, found `&Binding<&ThinSlice<Expr<'_>>>`
    |                      arguments to this method are incorrect
    |
    |
    = note: expected reference `&author::Binding<&[rustc_hir::Expr<'_>]>`
               found reference `&author::Binding<&ThinSlice<rustc_hir::Expr<'_>>>`
note: method defined here
    |
    |
242 |     fn slice<T>(&self, slice: &Binding<&[T]>, f: impl Fn(&Binding<&T>)) {

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:414:28
    |
    |
414 |                 self.slice(args, |e| self.expr(e));
    |                      ----- ^^^^ expected `&Binding<&[Expr<'_>]>`, found `&Binding<&ThinSlice<Expr<'_>>>`
    |                      arguments to this method are incorrect
    |
    |
    = note: expected reference `&author::Binding<&[rustc_hir::Expr<'_>]>`
               found reference `&author::Binding<&ThinSlice<rustc_hir::Expr<'_>>>`
note: method defined here
    |
    |
242 |     fn slice<T>(&self, slice: &Binding<&[T]>, f: impl Fn(&Binding<&T>)) {

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:436:26
    |
    |
436 |                 self.lit(lit);
    |                      --- ^^^ expected `&Binding<&Spanned<LitKind>>`, found `&Binding<&&Spanned<LitKind>>`
    |                      arguments to this method are incorrect
    |
    |
    = note: expected reference `&author::Binding<&rustc_span::source_map::Spanned<rustc_ast::LitKind>>`
               found reference `&author::Binding<&&rustc_span::source_map::Spanned<rustc_ast::LitKind>>`
note: method defined here
    |
    |
276 |     fn lit(&self, lit: &Binding<&Lit>) {


error[E0282]: type annotations needed for `&author::Binding<&T>`
    |
    |
88  |             value: $binding.value.$field,
    |                    --------------------- type must be known at this point
...
634 |                 self.slice(fields, |field| {
    |
    |
help: consider giving this closure parameter an explicit type, where the type for type parameter `T` is specified
    |
634 |                 self.slice(fields, |field: &author::Binding<&T>| {


error[E0609]: no field `ident` on type `&_`
    |
    |
635 |                     self.ident(field!(field.ident));

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:642:28
    |
    |
642 |                 self.slice(fields, |pat| self.pat(pat));
    |                      ----- ^^^^^^ expected `&Binding<&[Pat<'_>]>`, found `&Binding<&ThinSlice<Pat<'_>>>`
    |                      arguments to this method are incorrect
    |
    |
    = note: expected reference `&author::Binding<&[rustc_hir::Pat<'_>]>`
               found reference `&author::Binding<&ThinSlice<rustc_hir::Pat<'_>>>`
note: method defined here
    |
    |
242 |     fn slice<T>(&self, slice: &Binding<&[T]>, f: impl Fn(&Binding<&T>)) {

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:648:28
    |
    |
648 |                 self.slice(fields, |pat| self.pat(pat));
    |                      ----- ^^^^^^ expected `&Binding<&[Pat<'_>]>`, found `&Binding<&ThinSlice<Pat<'_>>>`
    |                      arguments to this method are incorrect
    |
    |
    = note: expected reference `&author::Binding<&[rustc_hir::Pat<'_>]>`
               found reference `&author::Binding<&ThinSlice<rustc_hir::Pat<'_>>>`
note: method defined here
    |
    |
242 |     fn slice<T>(&self, slice: &Binding<&[T]>, f: impl Fn(&Binding<&T>)) {

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:658:28
    |
    |
658 |                 self.slice(fields, |field| self.pat(field));
    |                      ----- ^^^^^^ expected `&Binding<&[Pat<'_>]>`, found `&Binding<&ThinSlice<Pat<'_>>>`
    |                      arguments to this method are incorrect
    |
    |
    = note: expected reference `&author::Binding<&[rustc_hir::Pat<'_>]>`
               found reference `&author::Binding<&ThinSlice<rustc_hir::Pat<'_>>>`
note: method defined here
    |
    |
242 |     fn slice<T>(&self, slice: &Binding<&[T]>, f: impl Fn(&Binding<&T>)) {


error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
46 |             && let ExprKind::MethodCall(method_segment, recv, [], _) = condition.kind
   |                                                               ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/bool_assert_comparison.rs:44:14
    |
44  |         Some(b)
44  |         Some(b)
    |         ---- ^ expected `bool`, found `&bool`
    |         arguments to this enum variant are incorrect
    |
    |
help: the type constructed contains `&bool` due to the type of the argument passed
    |
44  |         Some(b)
    |         ^^^^^-^
    |              |
    |              |
    |              this argument influences the type of `Some`
   --> /checkout/library/core/src/option.rs:572:5
    |
572 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^
    |     ^^^^
help: consider dereferencing the borrow
    |
44  |         Some(*b)
    |              +

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   --> src/tools/clippy/clippy_lints/src/booleans.rs:273:46
    |
273 |         ExprKind::MethodCall(path, receiver, [], _) => {
    |                                              ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
43 |         if let ExprKind::Call(box_new, [arg]) = expr.kind
   |                                        ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
17 |         && let ExprKind::MethodCall(method_name, receiver, [], _) = cast_expr.peel_blocks().kind
   |                                                            ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
49 |         ExprKind::MethodCall(method, left, [right], _) => {
   |                                            ^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
60 |         ExprKind::MethodCall(method, _, [lo, hi], _) => {
   |                                         ^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
69 |         ExprKind::MethodCall(method, _value, [], _) => {
   |                                              ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::GenericArg<'_>>`
   |
   |
24 |             && let [GenericArg::Type(cast_to)] = generic_args.args
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::GenericArg<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
76 |         ExprKind::Call(func, [arg, ..]) if arg.hir_id == e.hir_id => {
   |                              ^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
33 |         if let ExprKind::Call(fun, [ptr_arg, len_arg]) = cast_expr.peel_blocks().kind;
   |                                    ^^^^^^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
263 |             if let ExprKind::Call(from_func, [limit]) = &expr.kind;
    |                                              ^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::PathSegment<'_>>`
    |
    |
305 |         if let [int] = tp.segments;
    |                ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::PathSegment<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::PathSegment<'_>>`
    |
    |
319 |         if let [ty] = path.segments;
    |                ^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::PathSegment<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/create_dir.rs:37:41
   |
37 |             if let ExprKind::Call(func, [arg, ..]) = expr.kind;
   |                                         ^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
36 |         if let ExprKind::Call(iter_expr, []) = &expr.kind
   |                                          ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
605 |         ExprKind::MethodCall(_, arg, [], _) => (typeck.type_dependent_def_id(expr.hir_id)?, arg),
    |                                      ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
612 |             [arg],
    |             ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::PathSegment<'_>>`
     |
     |
1015 |                         segments: [.., path], ..
     |                                   ^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::PathSegment<'_>>`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/derivable_impls.rs:88:55
    |
    |
88  |             let args = args.map(|a| a.args).unwrap_or(&[]);
    |                                             --------- ^^^ expected `&ThinSlice<GenericArg<'_>>`, found `&[_; 0]`
    |                                             arguments to this method are incorrect
    |
    |
    = note: expected reference `&ThinSlice<rustc_hir::GenericArg<'_>>`
               found reference `&[_; 0]`
help: the return type of this call is `&[_; 0]` due to the type of the argument passed
    |
    |
88  |             let args = args.map(|a| a.args).unwrap_or(&[]);
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---^
    |                                                       |
    |                                                       this argument influences the return type of `unwrap_or`
note: method defined here
    |
973 |     pub const fn unwrap_or(self, default: T) -> T
    |                  ^^^^^^^^^


error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
200 |         if let ExprKind::Call(path, [arg]) = expr.kind
    |                                     ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   --> src/tools/clippy/clippy_lints/src/entry.rs:235:13
235 | /             [
236 | |                 Expr {
236 | |                 Expr {
237 | |                     kind: ExprKind::AddrOf(_, _, key),
238 | |                     span: key_span,
240 | |                 },
241 | |             ],
241 | |             ],
    | |_____________^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   --> src/tools/clippy/clippy_lints/src/entry.rs:269:41
    |
269 |     if let ExprKind::MethodCall(_, map, [key, value], _) = expr.kind {
    |                                         ^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
48 |             if let ExprKind::MethodCall(unwrap_fun, write_call, [], _) = expr.kind;
   |                                                                 ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
51 |             if let ExprKind::MethodCall(write_fun, write_recv, [write_arg], _) = look_in_block(cx, &write_call.kind);
   |                                                                ^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
377 |                 largs_0, [largs_1, ..],
    |                          ^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
382 |                 rargs_0, [rargs_1, ..],
    |                          ^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
86 |                     ExprKind::Call(func, []) if is_path_diagnostic_item(cx, func, sym::ptr_null) => {
   |                                          ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::PathSegment<'_>>`
    |
    |
342 |         && let [segment] = path.segments
    |                ^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::PathSegment<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
401 |         if let ExprKind::MethodCall(_, receiver, [], to_string_span) = value.kind;
    |                                                  ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::PathSegment<'_>>`
    |
    |
151 |         if let [segment] = path.segments;
    |                ^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::PathSegment<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
57 |             ExprKind::MethodCall(_, _, [arg], _) => {
   |                                        ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::GenericArg<'_>>`
   |
   |
78 |             && let Some(GenericArgs { args: [GenericArg::Type(target_ty)], .. }) = into_trait_seg.args
   |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^          ------------------- help: consider using `as_deref` here: `into_trait_seg.args.as_deref()`
   |                                             |
   |                                             pattern cannot match with input type `ThinSlice<rustc_hir::GenericArg<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
43 |         if let ExprKind::Call(box_from_raw, [arg]) = expr.kind
   |                                             ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
50 |             if let ExprKind::Call(maybe_path, [src, radix]) = &exp.kind;
   |                                               ^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/if_then_some_else_none.rs:82:46
   |
82 |             && let ExprKind::Call(then_call, [then_arg]) = then_expr.kind
   |                                              ^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
111 |                     ExprKind::Call(func, []) => {
    |                                          ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
120 |     if let ExprKind::Call(fn_expr, []) = expr_block.kind
    |                                    ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
33 |         if let ExprKind::Call(ctor, [inner_ret]) = ret_value.kind;
   |                                     ^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
122 |             if let ExprKind::MethodCall(method, recv, [], _) = end.kind;
    |                                                       ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
335 |         if let ExprKind::MethodCall(method, arg, [], _) = expr.kind;
    |                                                  ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   --> src/tools/clippy/clippy_lints/src/loops/needless_range_loop.rs:191:51
    |
191 |         if let ExprKind::MethodCall(method, recv, [], _) = expr.kind;
    |                                                   ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   --> src/tools/clippy/clippy_lints/src/loops/needless_range_loop.rs:306:55
    |
306 |             if let ExprKind::MethodCall(meth, args_0, [args_1, ..], _) = &expr.kind;
    |                                                       ^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
43 |             [],
43 |             [],
   |             ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
52 |             [],
52 |             [],
   |             ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
61 |             [],
61 |             [],
   |             ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/loops/while_let_on_iterator.rs:24:61
   |
24 |         if let ExprKind::MethodCall(method_name, iter_expr, [], _) = let_expr.kind;
   |                                                             ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
673 |     if let ExprKind::MethodCall(method, self_arg, [], _) = arg.kind {
    |                                                   ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
142 |     } else if let ExprKind::MethodCall(path, _, [], _) = expr.kind
    |                                                 ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
268 |     if let ExprKind::MethodCall(seg_second, receiver, [arg_second], _) = &expr.kind
    |                                                       ^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
270 |         && let ExprKind::MethodCall(seg_first, input, [arg_first], _) = &receiver.kind
    |                                                       ^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
365 |     if let ExprKind::Call(outer_fn, [first, second]) = &expr.kind {
    |                                     ^^^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
332 |         if let ExprKind::Call(inner_fn, [first, second]) = &inner_call.kind
    |                                         ^^^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
93 |         } else if let ExprKind::MethodCall(path, receiver, [arg], ..) = expr.kind
   |                                                            ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::PathSegment<'_>>`
    |
    |
331 |             if let [path_seg] = path.segments;
    |                    ^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::PathSegment<'_>>`

---

error[E0308]: `match` arms have incompatible types
   --> src/tools/clippy/clippy_lints/src/matches/match_same_arms.rs:250:29
    |
248 |                   let (front, back) = match wild_idx.as_opt_usize() {
    |  _____________________________________-
249 | |                     Some(i) => pats.split_at(i),
    | |                                ---------------- this is found to be of type `(&[rustc_hir::Pat<'_>], &[rustc_hir::Pat<'_>])`
250 | |                     None => (pats, [].as_slice()),
    | |                             ^^^^^^^^^^^^^^^^^^^^^ expected `(&[Pat<'_>], &[Pat<'_>])`, found `(&ThinSlice<Pat<'_>>, &[_])`
    | |_________________- `match` arms have incompatible types
    |
    |
    = note: expected tuple `(&[rustc_hir::Pat<'_>], &[rustc_hir::Pat<'_>])`
               found tuple `(&ThinSlice<rustc_hir::Pat<'_>>, &[_])`
error[E0308]: `match` arms have incompatible types
   --> src/tools/clippy/clippy_lints/src/matches/match_same_arms.rs:270:29
    |
    |
268 |                   let (front, back) = match wild_idx.as_opt_usize() {
    |  _____________________________________-
269 | |                     Some(i) => pats.split_at(i),
    | |                                ---------------- this is found to be of type `(&[rustc_hir::Pat<'_>], &[rustc_hir::Pat<'_>])`
270 | |                     None => (pats, [].as_slice()),
    | |                             ^^^^^^^^^^^^^^^^^^^^^ expected `(&[Pat<'_>], &[Pat<'_>])`, found `(&ThinSlice<Pat<'_>>, &[_])`
    | |_________________- `match` arms have incompatible types
    |
    |
    = note: expected tuple `(&[rustc_hir::Pat<'_>], &[rustc_hir::Pat<'_>])`
               found tuple `(&ThinSlice<rustc_hir::Pat<'_>>, &[_])`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
51 |             ExprKind::MethodCall(segment, receiver, [], _) if self.case_altered(segment.ident.as_str(), receiver) => {},
   |                                                     ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::PathSegment<'_>>`
    |
    |
184 |                     segments: [first_seg, ..],
    |                               ^^^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::PathSegment<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Pat<'_>>`
   |
   |
66 |         PatKind::TupleStruct(ref qpath, [sub_pat], _) => {
   |                                         ^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Pat<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
93 |         (MatchSource::ForLoopDesugar, ExprKind::Call(_, [e])) => e,
   |                                                         ^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
26 |         if let ExprKind::Call(match_fun, [try_arg, ..]) = scrutinee.kind;
   |                                          ^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
29 |         if let ExprKind::Call(err_fun, [err_arg, ..]) = try_arg.kind;
   |                                        ^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
32 |         if let ExprKind::Call(path_expr, [ref first_arg, ..]) = e.kind {
   |                                          ^^^^^^^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
164 |         if let ExprKind::Call(repl_func, []) = src.kind;
    |                                          ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
244 |             if let ExprKind::Call(func, [dest, src]) = expr.kind;
    |                                         ^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
73 |             if let hir::ExprKind::Call(some_expr, [inner_expr]) = closure_expr.kind;
   |                                                   ^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
106 |                 if let hir::ExprKind::Call(func_path, [arg]) = ret_expr.kind;
    |                                                       ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/methods/bytecount.rs:45:72
   |
45 |             let haystack = if let ExprKind::MethodCall(path, receiver, [], _) =
   |                                                                        ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/methods/chars_cmp.rs:21:41
   |
21 |         if let hir::ExprKind::Call(fun, [arg_char]) = info.other.kind;
   |                                         ^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
32 |                 hir::ExprKind::MethodCall(method_name, receiver, [], ..) => {
   |                                                                  ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
17 |         if let ExprKind::MethodCall(src_method, drain_vec, [drain_arg], _) = &arg.kind;
   |                                                            ^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
109 |         if let ExprKind::MethodCall(path, filter_arg, [], _) = filter_body.value.kind;
    |                                                       ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
132 |             ExprKind::MethodCall(method, original_arg, [], _) if acceptable_methods(method) => {
    |                                                        ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
24 |         && let ExprKind::MethodCall(lhs_path, lhs_recv, [], _) = &lhs.kind
   |                                                         ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Pat<'_>>`
  --> src/tools/clippy/clippy_lints/src/methods/iter_kv_map.rs:31:31
   |
31 |         if let PatKind::Tuple([key_pat, val_pat], _) = p.pat.kind;
   |                               ^^^^^^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Pat<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::PathSegment<'_>>`
  --> src/tools/clippy/clippy_lints/src/methods/iter_kv_map.rs:49:24
   |
49 |                 if let [local_ident] = path.segments;
   |                        ^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::PathSegment<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/methods/iter_on_single_or_empty_collections.rs:33:27
   |
33 |         ExprKind::Call(f, [arg]) if is_res_lang_ctor(cx, path_res(cx, f), OptionSome) => Some(arg),
   |                           ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/methods/iter_with_drain.rs:38:61
   |
38 |                 && let ExprKind::MethodCall(name, self_arg, [], _) = e.kind
   |                                                             ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
25 |         if let ExprKind::Call(err_path, [err_arg]) = or_expr.kind;
   |                                         ^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
54 |                 && let ExprKind::Call(callee, [ok_arg]) = body.value.kind
   |                                               ^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
59 |         if let ExprKind::Call(repeat_fn, [repeat_arg]) = take_self_arg.kind;
   |                                          ^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
45 |                         hir::ExprKind::MethodCall(method, obj, [], _) => if_chain! {
   |                                                                ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
35 |                 if let ExprKind::MethodCall(name, _, args @ ([] | [_]), _) = parent.kind {
   |                                                              ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
35 |                 if let ExprKind::MethodCall(name, _, args @ ([] | [_]), _) = parent.kind {
   |                                                                   ^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
261 |         if let ExprKind::MethodCall(method_name, recv, [args @ ..], _) = &expr.kind {
    |                                                        ^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/methods/open_options.rs:51:28
   |
   |
51 |                         if lit { Argument::True } else { Argument::False }
   |                            ^^^ expected `bool`, found `&bool`
help: consider dereferencing the borrow
   |
   |
51 |                         if *lit { Argument::True } else { Argument::False }


error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/methods/option_as_ref_deref.rs:60:56
   |
60 |                 hir::ExprKind::MethodCall(_, receiver, [], _) => {
   |                                                        ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/methods/or_then_unwrap.rs:60:38
   |
60 |     if let ExprKind::Call(some_expr, [arg]) = expr.kind
   |                                      ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/methods/range_zip_with_len.rs:19:57
   |
19 |         if let ExprKind::MethodCall(len_path, len_recv, [], _) = end.kind;
   |                                                         ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   --> src/tools/clippy/clippy_lints/src/methods/str_splitn.rs:292:47
    |
292 |             let ExprKind::MethodCall(name, _, [args @ ..], _) = e.kind else {
    |                                               ^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   --> src/tools/clippy/clippy_lints/src/methods/str_splitn.rs:325:75
    |
325 | ...                   if let ExprKind::MethodCall(next_name, _, [], _) = next_expr.kind;
    |                                                                 ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   --> src/tools/clippy/clippy_lints/src/methods/str_splitn.rs:365:43
    |
365 |             ExprKind::MethodCall(name, _, [], _)
    |                                           ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/methods/unnecessary_iter_cloned.rs:46:81
   |
46 |                 if let ExprKind::MethodCall(maybe_iter_method_name, collection, [], _) = receiver.kind;
   |                                                                                 ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   --> src/tools/clippy/clippy_lints/src/methods/unnecessary_sort_by.rs:128:61
    |
128 |         if let ExprKind::MethodCall(method_path, left_expr, [right_expr], _) = closure_body.value.kind;
    |                                                             ^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::PathSegment<'_>>`
   --> src/tools/clippy/clippy_lints/src/methods/unnecessary_sort_by.rs:148:31
    |
148 |                     segments: [PathSegment { ident: left_name, .. }], ..
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::PathSegment<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
     |
     |
3806 |                     if let ExprKind::MethodCall(name, iter_recv, [], _) = recv.kind
     |                                                                  ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
85 |         ExprKind::MethodCall(path, receiver, args @ [_], _) => {
   |                                                     ^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/needless_for_each.rs:58:69
   |
58 |             if let ExprKind::MethodCall(method_name, for_each_recv, [for_each_arg], _) = expr.kind;
   |                                                                     ^^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/needless_for_each.rs:63:55
   |
63 |             if let ExprKind::MethodCall(_, iter_recv, [], _) = for_each_recv.kind;
   |                                                       ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   --> src/tools/clippy/clippy_lints/src/needless_question_mark.rs:115:37
    |
115 |         if let ExprKind::Call(path, [arg]) = expr.kind;
    |                                     ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   --> src/tools/clippy/clippy_lints/src/needless_question_mark.rs:126:39
    |
126 |         if let ExprKind::Call(called, [inner_expr]) = &inner_expr_with_q.kind;
    |                                       ^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
47 |             ExprKind::MethodCall(path, func, [param], _) => {
   |                                              ^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
69 |             ExprKind::Call(func, [param]) => {
   |                                  ^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
41 |         ExprKind::MethodCall(_, arg, [], _)
   |                                      ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
51 |         ExprKind::Call(path, [arg])
   |                              ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
  --> src/tools/clippy/clippy_lints/src/operators/duration_subsec.rs:20:60
   |
20 |         && let ExprKind::MethodCall(method_path, self_arg, [], _) = left.kind
   |                                                            ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Pat<'_>>`
    |
    |
190 |     if let PatKind::TupleStruct(ref qpath, [inner_pat], ..) = pat.kind {
    |                                            ^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Pat<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Pat<'_>>`
    |
    |
248 |         PatKind::TupleStruct(ref qpath, [first_pat], _) => {
    |                                         ^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Pat<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Pat<'_>>`
    |
    |
175 |             PatKind::Or([p, ..]) => p,
    |                         ^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Pat<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
33 |         if let ExprKind::MethodCall(path, receiver, [arg], _) = &expr.kind
   |                                                     ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
719 |     if let ExprKind::Call(pathexp, []) = expr.kind {
    |                                    ^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
94 |     if let ExprKind::MethodCall(path_segment, arg_0, [arg_1, ..], _) = &expr.kind {
   |                                                      ^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Pat<'_>>`
    |
    |
126 |         if let PatKind::TupleStruct(ref path1, [field], ddpos) = let_pat.kind;
    |                                                ^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Pat<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
63 |                     if let ExprKind::MethodCall(path, _, [arg], _) = expr.kind
   |                                                          ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::PathSegment<'_>>`
   |
   |
68 |                         && let [inner_seg] = inner_path.segments
   |                                ^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::PathSegment<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
63 |             if let ExprKind::Call(fun, [arg]) = expr.kind;
   |                                        ^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::PathSegment<'_>>`
   |
   |
98 |             && let [fun_ident, ..] = fun_path.segments
   |                    ^^^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::PathSegment<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
100 |             && let [first_arg, ..] = args
    |                    ^^^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::PathSegment<'_>>`
    |
    |
102 |             && let [first_arg_ps, .. ] = arg_path.segments
    |                    ^^^^^^^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::PathSegment<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
   |
   |
98 |         if let ExprKind::Call(func, [.., count]) = expr.kind;
   |                                     ^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
111 |         if let ExprKind::MethodCall(method_path, ptr_self, [.., count], _) = expr.kind;
    |                                                            ^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`

error[E0529]: expected an array or slice, found `ThinSlice<rustc_hir::Expr<'_>>`
    |
    |
239 |             if let ExprKind::MethodCall(take_path, recv, [len_arg, ..], _) = expr.kind;
    |                                                          ^^^^^^^^^^^^^ pattern cannot match with input type `ThinSlice<rustc_hir::Expr<'_>>`
error[E0308]: `match` arms have incompatible types
   --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/if_chain-1.0.0/src/lib.rs:207:9
    |
156 |   macro_rules! if_chain {
156 |   macro_rules! if_chain {
    |   --------------------- in this expansion of `if_chain!` (#1)
157 |       ($($tt:tt)*) => {
158 |           __if_chain! { @init () $($tt)* }
...
164 |   macro_rules! __if_chain {
    |   -----------------------
    |   |
    |   |
    |   in this expansion of `__if_chain!` (#2)
    |   in this expansion of `__if_chain!` (#3)
    |   in this expansion of `__if_chain!` (#4)
    |   in this expansion of `__if_chain!` (#5)
...
167 |           __if_chain! { @expand { $($other)* } $($tt)* then { $($then)* } }
...
...
176 |           __if_chain! { @init ($($tt)* $head) $($tail)* }
    |           |
    |           in this macro invocation (#3)
    |           in this macro invocation (#4)
...
...
207 | /         if let $pat = $expr {
208 | |             __if_chain! { @expand { $($other)+ } $($tt)+ }
209 | |         } else {
210 | |             $($other)+
211 | |         }
    | |_________^ expected `Option<(bool, &Expr<'_>, &...)>`, found `Option<(bool, Expr<'_>, Expr<'_>)>`
...
228 | /         if $expr {
229 | |             __if_chain! { @expand { $($other)+ } $($tt)+ }
230 | |         } else {
231 | |             $($other)+
232 | |         }
    | |_________- this is found to be of type `std::option::Option<(bool, &rustc_hir::Expr<'_>, &rustc_hir::Expr<'_>)>`
   ::: src/tools/clippy/clippy_lints/src/to_digit_is_some.rs:46:36
    |
    |
46  |                   let match_result = match &to_digit_expr.kind {
    |                                      ------------------------- `match` arms have incompatible types
62  | /                         if_chain! {
62  | /                         if_chain! {
63  | |                             if let [char_arg, radix_arg] = *to_digit_args.as_slice();
64  | |                             if let hir::ExprKind::Path(to_digits_path) = &to_digits_call.kind;
65  | |                             if let to_digits_call_res = cx.qpath_res(to_digits_path, to_digits_call.hir_id);
72  | |                             }
73  | |                         }
    | |_________________________- in this macro invocation (#1)
    |
    |
    = note: expected enum `std::option::Option<(_, &rustc_hir::Expr<'_>, &rustc_hir::Expr<'_>)>`
               found enum `std::option::Option<(_, rustc_hir::Expr<'_>, rustc_hir::Expr<'_>)>`
Some errors have detailed explanations: E0282, E0308, E0529, E0609.
For more information about an error, try `rustc --explain E0282`.
error: could not compile `clippy_lints` due to 163 previous errors
warning: build failed, waiting for other jobs to finish...
