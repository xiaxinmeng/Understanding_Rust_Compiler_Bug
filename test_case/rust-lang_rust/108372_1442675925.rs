plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling clippy_utils v0.1.69 (/checkout/src/tools/clippy/clippy_utils)
error: binary operation on reference to `Copy` type `rustc_ast::BindingAnnotation`
  --> src/tools/clippy/clippy_utils/src/ast_utils.rs:40:54
   |
40 |         (Ident(b1, i1, s1), Ident(b2, i2, s2)) => b1 == b2 && eq_id(*i1, *i2) && both(s1, s2, |l, r| eq_pat(l, r)),
   |
   |
   = note: `rustc_ast::BindingAnnotation` takes `2` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
   = note: `-D ref-binop-on-copy-type` implied by `-D warnings`
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
   |
40 |         (Ident(b1, i1, s1), Ident(b2, i2, s2)) => *b1 == *b2 && eq_id(*i1, *i2) && both(s1, s2, |l, r| eq_pat(l, r)),

error: binary operation on reference to `Copy` type `bool`
  --> src/tools/clippy/clippy_utils/src/ast_utils.rs:53:16
   |
   |
53 |             lr == rr && eq_maybe_qself(lqself, rqself) && eq_path(lp, rp) && unordered_over(lfs, rfs, eq_field_pat)
   |
   |
   = note: `bool` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
   |
53 |             *lr == *rr && eq_maybe_qself(lqself, rqself) && eq_path(lp, rp) && unordered_over(lfs, rfs, eq_field_pat)

error: binary operation on reference to `Copy` type `rustc_ast::CaptureBy`
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:206:23
    |
    |
206 |                 && lc == rc
    |
    |
    = note: `rustc_ast::CaptureBy` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
206 |                 && *lc == *rc

error: binary operation on reference to `Copy` type `rustc_ast::Movability`
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:208:23
    |
    |
208 |                 && lm == rm
    |
    |
    = note: `rustc_ast::Movability` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
208 |                 && *lm == *rm

error: binary operation on reference to `Copy` type `rustc_ast::CaptureBy`
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:212:52
    |
    |
212 |         (Async(lc, _, lb), Async(rc, _, rb)) => lc == rc && eq_block(lb, rb),
    |
    |
    = note: `rustc_ast::CaptureBy` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
212 |         (Async(lc, _, lb), Async(rc, _, rb)) => *lc == *rc && eq_block(lb, rb),

error: binary operation on reference to `Copy` type `rustc_ast::RangeLimits`
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:213:54
    |
    |
213 |         (Range(lf, lt, ll), Range(rf, rt, rl)) => ll == rl && eq_expr_opt(lf, rf) && eq_expr_opt(lt, rt),
    |
    |
    = note: `rustc_ast::RangeLimits` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
213 |         (Range(lf, lt, ll), Range(rf, rt, rl)) => *ll == *rl && eq_expr_opt(lf, rf) && eq_expr_opt(lt, rt),

error: binary operation on reference to `Copy` type `rustc_ast::BorrowKind`
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:214:59
    |
    |
214 |         (AddrOf(lbk, lm, le), AddrOf(rbk, rm, re)) => lbk == rbk && lm == rm && eq_expr(le, re),
    |
    |
    = note: `rustc_ast::BorrowKind` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
214 |         (AddrOf(lbk, lm, le), AddrOf(rbk, rm, re)) => *lbk == *rbk && lm == rm && eq_expr(le, re),

error: binary operation on reference to `Copy` type `rustc_ast::Mutability`
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:214:72
    |
    |
214 |         (AddrOf(lbk, lm, le), AddrOf(rbk, rm, re)) => lbk == rbk && lm == rm && eq_expr(le, re),
    |
    |
    = note: `rustc_ast::Mutability` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
214 |         (AddrOf(lbk, lm, le), AddrOf(rbk, rm, re)) => lbk == rbk && *lm == *rm && eq_expr(le, re),

error: binary operation on reference to `Copy` type `std::option::Option<rustc_span::Symbol>`
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:287:47
    |
    |
287 |         (ExternCrate(l), ExternCrate(r)) => l == r,
    |
    |
    = note: `std::option::Option<rustc_span::Symbol>` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
287 |         (ExternCrate(l), ExternCrate(r)) => *l == *r,

error: binary operation on reference to `Copy` type `rustc_ast::Mutability`
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:289:56
    |
    |
289 |         (Static(lt, lm, le), Static(rt, rm, re)) => lm == rm && eq_ty(lt, rt) && eq_expr_opt(le, re),
    |
    |
    = note: `rustc_ast::Mutability` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
289 |         (Static(lt, lm, le), Static(rt, rm, re)) => *lm == *rm && eq_ty(lt, rt) && eq_expr_opt(le, re),

error: binary operation on reference to `Copy` type `rustc_ast::Inline`
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:311:33
    |
    |
311 |                         linline == rinline && over(litems, ritems, |l, r| eq_item(l, r, eq_item_kind))
    |
    |
    = note: `rustc_ast::Inline` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
311 |                         *linline == *rinline && over(litems, ritems, |l, r| eq_item(l, r, eq_item_kind))


error: binary operation on reference to `Copy` type `rustc_ast::IsAuto`
    |
    |
361 |             la == ra
    |
    |
    = note: `rustc_ast::IsAuto` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
361 |             *la == *ra

error: binary operation on reference to `Copy` type `rustc_ast::Mutability`
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:408:56
    |
    |
408 |         (Static(lt, lm, le), Static(rt, rm, re)) => lm == rm && eq_ty(lt, rt) && eq_expr_opt(le, re),
    |
    |
    = note: `rustc_ast::Mutability` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
408 |         (Static(lt, lm, le), Static(rt, rm, re)) => *lm == *rm && eq_ty(lt, rt) && eq_expr_opt(le, re),

error: binary operation on reference to `Copy` type `rustc_ast::TraitObjectSyntax`
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:640:58
    |
    |
640 |         (TraitObject(lg, ls), TraitObject(rg, rs)) => ls == rs && over(lg, rg, eq_generic_bound),
    |
    |
    = note: `rustc_ast::TraitObjectSyntax` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
640 |         (TraitObject(lg, ls), TraitObject(rg, rs)) => *ls == *rs && over(lg, rg, eq_generic_bound),


error: binary operation on reference to `Copy` type `rustc_ast::TraitBoundModifier`
    |
    |
696 |         (Trait(ptr1, tbm1), Trait(ptr2, tbm2)) => tbm1 == tbm2 && eq_poly_ref_trait(ptr1, ptr2),
    |
    |
    = note: `rustc_ast::TraitBoundModifier` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
696 |         (Trait(ptr1, tbm1), Trait(ptr2, tbm2)) => *tbm1 == *tbm2 && eq_poly_ref_trait(ptr1, ptr2),

error: binary operation on reference to `Copy` type `rustc_ast::token::CommentKind`
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:728:60
    |
    |
728 |             (DocComment(l1, l2), DocComment(r1, r2)) => l1 == r1 && l2 == r2,
    |
    |
    = note: `rustc_ast::token::CommentKind` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
728 |             (DocComment(l1, l2), DocComment(r1, r2)) => *l1 == *r1 && l2 == r2,

error: binary operation on reference to `Copy` type `rustc_span::Symbol`
   --> src/tools/clippy/clippy_utils/src/ast_utils.rs:728:72
    |
    |
728 |             (DocComment(l1, l2), DocComment(r1, r2)) => l1 == r1 && l2 == r2,
    |
    |
    = note: `rustc_span::Symbol` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
728 |             (DocComment(l1, l2), DocComment(r1, r2)) => l1 == r1 && *l2 == *r2,

error: binary operation on reference to `Copy` type `u64`
  --> src/tools/clippy/clippy_utils/src/consts.rs:72:64
   |
   |
72 |             (Self::Repeat(lv, ls), Self::Repeat(rv, rs)) => ls == rs && lv == rv,
   |
   |
   = note: `u64` takes `8` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
   |
72 |             (Self::Repeat(lv, ls), Self::Repeat(rv, rs)) => *ls == *rs && lv == rv,

error: binary operation on reference to `Copy` type `rustc_hir::LoopSource`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:271:21
    |
    |
271 |                 lls == rls && self.eq_block(lb, rb) && both(ll, rl, |l, r| l.ident.name == r.ident.name)
    |
    |
    = note: `rustc_hir::LoopSource` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
271 |                 *lls == *rls && self.eq_block(lb, rb) && both(ll, rl, |l, r| l.ident.name == r.ident.name)


error: binary operation on reference to `Copy` type `rustc_hir::MatchSource`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:274:20
274 |                 ls == rs
    |                    ^^
    |
    |
    = note: `rustc_hir::MatchSource` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
274 |                 *ls == *rs

error: binary operation on reference to `Copy` type `rustc_ast::Mutability`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:371:74
    |
    |
371 |             (&PatKind::Ref(le, ref lm), &PatKind::Ref(re, ref rm)) => lm == rm && self.eq_pat(le, re),
    |
    |
    = note: `rustc_ast::Mutability` takes `1` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
371 |             (&PatKind::Ref(le, ref lm), &PatKind::Ref(re, ref rm)) => *lm == *rm && self.eq_pat(le, re),

error: binary operation on reference to `Copy` type `char`
   --> src/tools/clippy/clippy_utils/src/str_utils.rs:201:45
    |
    |
201 |         .take_while(|((_, c1), (_, c2))| c1 == c2)
    |
    |
    = note: `char` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
201 |         .take_while(|((_, c1), (_, c2))| *c1 == *c2)

error: binary operation on reference to `Copy` type `char`
   --> src/tools/clippy/clippy_utils/src/str_utils.rs:232:45
    |
    |
232 |         .take_while(|((_, c1), (_, c2))| c1 == c2)
    |
    |
    = note: `char` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
232 |         .take_while(|((_, c1), (_, c2))| *c1 == *c2)

error: binary operation on reference to `Copy` type `rustc_middle::ty::AdtDef<'_>`
   --> src/tools/clippy/clippy_utils/src/ty.rs:521:22
    |
    |
521 |             if did_a != did_b {
    |
    |
    = note: `rustc_middle::ty::AdtDef<'_>` takes `8` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
    |
521 |             if *did_a != *did_b {

error: could not compile `clippy_utils` due to 24 previous errors
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
thread 'main' panicked at 'in-tree tool', test.rs:745:14
Build completed unsuccessfully in 0:00:12
