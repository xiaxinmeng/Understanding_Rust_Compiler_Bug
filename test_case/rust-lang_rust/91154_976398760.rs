plain
    Checking clippy_lints v0.1.58 (/checkout/src/tools/clippy/clippy_lints)
error: there is no argument named `node`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:187:43
    |
187 |             Some(node) => write!(f, "Some({node})"),
    |
    |
    = help: if you intended to capture `node` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `s`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:214:27
    |
    |
214 |             n => format!("{s}{n}"),
    |
    |
    = help: if you intended to capture `s` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `n`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:214:30
    |
    |
214 |             n => format!("{s}{n}"),
    |
    |
    = help: if you intended to capture `n` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `option`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:227:30
    |
    |
227 |             None => out!("if {option}.is_none();"),
    |
    |
    = help: if you intended to capture `option` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `value`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:230:35
    |
    |
230 |                 out!("if let Some({value}) = {option};");
    |
    |
    = help: if you intended to capture `value` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `option`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:230:46
    |
    |
230 |                 out!("if let Some({value}) = {option};");
    |
    |
    = help: if you intended to capture `option` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `slice`
    |
    |
238 |             out!("if {slice}.is_empty();");
    |
    |
    = help: if you intended to capture `slice` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `slice`
    |
    |
240 |             out!("if {slice}.len() == {};", slice.value.len());
    |
    |
    = help: if you intended to capture `slice` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `slice`
    |
    |
242 |                 let name = format!("{slice}[{i}]");
    |
    |
    = help: if you intended to capture `slice` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `i`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:242:45
    |
    |
242 |                 let name = format!("{slice}[{i}]");
    |
    |
    = help: if you intended to capture `i` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `ident`
    |
    |
255 |         out!("if {ident}.as_str() == {:?};", ident.value.as_str());
    |
    |
    = help: if you intended to capture `ident` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `symbol`
    |
    |
259 |         out!("if {symbol}.as_str() == {:?};", symbol.value.as_str());
    |
    |
    = help: if you intended to capture `symbol` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `qpath`
    |
    |
264 |             out!("if matches!({qpath}, QPath::LangItem(LangItem::{lang_item:?}, _));");
    |
    |
    = help: if you intended to capture `qpath` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `lang_item`
    |
    |
264 |             out!("if matches!({qpath}, QPath::LangItem(LangItem::{lang_item:?}, _));");
    |
    |
    = help: if you intended to capture `lang_item` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `qpath`
    |
    |
266 |             out!("if match_qpath({qpath}, &[{}]);", path_to_string(qpath.value));
    |
    |
    = help: if you intended to capture `qpath` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `kind`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:271:49
    |
    |
271 |         let kind = |kind| out!("if let LitKind::{kind} = {lit}.node;");
    |
    |
    = help: if you intended to capture `kind` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `lit`
    |
    |
271 |         let kind = |kind| out!("if let LitKind::{kind} = {lit}.node;");
    |
    |
    = help: if you intended to capture `lit` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `val`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:277:47
    |
    |
277 |             LitKind::Bool(val) => kind!("Bool({val:?})"),
    |
    |
    = help: if you intended to capture `val` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `c`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:278:45
    |
    |
278 |             LitKind::Char(c) => kind!("Char({c:?})"),
    |
    |
    = help: if you intended to capture `c` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `val`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:279:45
    |
    |
279 |             LitKind::Err(val) => kind!("Err({val})"),
    |
    |
    = help: if you intended to capture `val` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `b`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:280:45
    |
    |
280 |             LitKind::Byte(b) => kind!("Byte({b})"),
    |
    |
    = help: if you intended to capture `b` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `int_ty`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:283:86
    |
    |
283 |                     LitIntType::Signed(int_ty) => format!("LitIntType::Signed(IntTy::{int_ty:?})"),
    |
    |
    = help: if you intended to capture `int_ty` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `uint_ty`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:284:92
    |
    |
284 |                     LitIntType::Unsigned(uint_ty) => format!("LitIntType::Unsigned(UintTy::{uint_ty:?})"),
    |
    |
    = help: if you intended to capture `uint_ty` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `i`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:287:28
    |
    |
287 |                 kind!("Int({i}, {int_ty})");
    |
    |
    = help: if you intended to capture `i` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `int_ty`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:287:33
    |
    |
287 |                 kind!("Int({i}, {int_ty})");
    |
    |
    = help: if you intended to capture `int_ty` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `suffix_ty`
    |
    |
291 |                     LitFloatType::Suffixed(suffix_ty) => format!("LitFloatType::Suffixed(FloatTy::{suffix_ty:?})"),
    |
    |
    = help: if you intended to capture `suffix_ty` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `float_ty`
    |
    |
294 |                 kind!("Float(_, {float_ty})");
    |
    |
    = help: if you intended to capture `float_ty` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `vec`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:298:36
    |
    |
298 |                 kind!("ByteStr(ref {vec})");
    |
    |
    = help: if you intended to capture `vec` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `vec`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:299:41
    |
    |
299 |                 out!("if let [{:?}] = **{vec};", vec.value);
    |
    |
    = help: if you intended to capture `vec` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `s`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:303:28
    |
    |
303 |                 kind!("Str({s}, _)");
    |
    |
    = help: if you intended to capture `s` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `arm`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:312:30
    |
    |
312 |             None => out!("if {arm}.guard.is_none();"),
    |
    |
    = help: if you intended to capture `arm` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `expr`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:315:45
    |
    |
315 |                 out!("if let Some(Guard::If({expr})) = {arm}.guard;");
    |
    |
    = help: if you intended to capture `expr` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `arm`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:315:56
    |
    |
315 |                 out!("if let Some(Guard::If({expr})) = {arm}.guard;");
    |
    |
    = help: if you intended to capture `arm` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `pat`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:320:48
    |
    |
320 |                 out!("if let Some(Guard::IfLet({pat}, {expr}) = {arm}.guard;");
    |
    |
    = help: if you intended to capture `pat` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `expr`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:320:55
    |
    |
320 |                 out!("if let Some(Guard::IfLet({pat}, {expr}) = {arm}.guard;");
    |
    |
    = help: if you intended to capture `expr` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `arm`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:320:65
    |
    |
320 |                 out!("if let Some(Guard::IfLet({pat}, {expr}) = {arm}.guard;");
    |
    |
    = help: if you intended to capture `arm` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `condition`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:333:58
    |
    |
333 |                 "if let Some(higher::While {{ condition: {condition}, body: {body} }}) \
    |
    |
    = help: if you intended to capture `condition` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `body`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:333:77
    |
    |
333 |                 "if let Some(higher::While {{ condition: {condition}, body: {body} }}) \
    |
    |
    = help: if you intended to capture `body` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `expr`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:334:38
    |
    |
334 |                 = higher::While::hir({expr});"
    |
    |
    = help: if you intended to capture `expr` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `let_pat`
    |
    |
349 |                 "if let Some(higher::WhileLet {{ let_pat: {let_pat}, let_expr: {let_expr}, if_then: {if_then} }}) \
    |
    |
    = help: if you intended to capture `let_pat` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `let_expr`
    |
    |
349 |                 "if let Some(higher::WhileLet {{ let_pat: {let_pat}, let_expr: {let_expr}, if_then: {if_then} }}) \
    |
    |
    = help: if you intended to capture `let_expr` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `if_then`
    |
    |
349 |                 "if let Some(higher::WhileLet {{ let_pat: {let_pat}, let_expr: {let_expr}, if_then: {if_then} }}) \
    |
    |
    = help: if you intended to capture `if_then` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `expr`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:350:41
    |
    |
350 |                 = higher::WhileLet::hir({expr});"
    |
    |
    = help: if you intended to capture `expr` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `pat`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:361:54
    |
    |
361 |                 "if let Some(higher::ForLoop {{ pat: {pat}, arg: {arg}, body: {body}, .. }}) \
    |
    |
    = help: if you intended to capture `pat` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `arg`
    |
    |
361 |                 "if let Some(higher::ForLoop {{ pat: {pat}, arg: {arg}, body: {body}, .. }}) \
    |
    |
    = help: if you intended to capture `arg` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `body`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:361:79
    |
    |
361 |                 "if let Some(higher::ForLoop {{ pat: {pat}, arg: {arg}, body: {body}, .. }}) \
    |
    |
    = help: if you intended to capture `body` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `expr`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:362:40
    |
    |
362 |                 = higher::ForLoop::hir({expr});"
    |
    |
    = help: if you intended to capture `expr` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `kind`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:370:50
    |
    |
370 |         let kind = |kind| out!("if let ExprKind::{kind} = {expr}.kind;");
    |
    |
    = help: if you intended to capture `kind` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `expr`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:370:59
    |
    |
370 |         let kind = |kind| out!("if let ExprKind::{kind} = {expr}.kind;");
    |
    |
    = help: if you intended to capture `expr` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `pat`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:378:28
    |
    |
378 |                 kind!("Let({pat}, {expr}, _)");
    |
    |
    = help: if you intended to capture `pat` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `expr`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:378:35
    |
    |
378 |                 kind!("Let({pat}, {expr}, _)");
    |
    |
    = help: if you intended to capture `expr` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `inner`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:384:28
    |
    |
384 |                 kind!("Box({inner})");
    |
    |
    = help: if you intended to capture `inner` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `elements`
    |
    |
389 |                 kind!("Array({elements})");
    |
    |
    = help: if you intended to capture `elements` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `func`
    |
    |
394 |                 kind!("Call({func}, {args})");
    |
    |
    = help: if you intended to capture `func` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `args`
    |
    |
394 |                 kind!("Call({func}, {args})");
    |
    |
    = help: if you intended to capture `args` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `method_name`
    |
    |
400 |                 kind!("MethodCall({method_name}, _, {args}, _)");
    |
    |
    = help: if you intended to capture `method_name` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `args`
    |
    |
400 |                 kind!("MethodCall({method_name}, _, {args}, _)");
    |
    |
    = help: if you intended to capture `args` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `elements`
    |
    |
406 |                 kind!("Tup({elements})");
    |
    |
    = help: if you intended to capture `elements` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `op`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:411:31
    |
    |
411 |                 kind!("Binary({op}, {left}, {right})");
    |
    |
    = help: if you intended to capture `op` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `left`
    |
    |
411 |                 kind!("Binary({op}, {left}, {right})");
    |
    |
    = help: if you intended to capture `left` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes

error: there is no argument named `right`
    |
    |
411 |                 kind!("Binary({op}, {left}, {right})");
    |
    |
    = help: if you intended to capture `right` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `op`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:412:45
    |
    |
412 |                 out!("if BinOpKind::{:?} == {op}.node;", op.value.node);
    |
    |
    = help: if you intended to capture `op` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: there is no argument named `op`
   --> src/tools/clippy/clippy_lints/src/utils/author.rs:418:36
    |
    |
