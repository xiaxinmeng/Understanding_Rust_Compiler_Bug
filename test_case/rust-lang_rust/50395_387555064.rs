plain
[00:55:51]    Compiling serde_derive v1.0.40
[00:56:13]    Compiling cargo_metadata v0.5.4
[00:56:28] [RUSTC-TIMING] cargo_metadata test:false 15.065
[00:56:28]    Compiling clippy_lints v0.0.197 (file:///checkout/src/tools/clippy/clippy_lints)
[00:56:32] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:32]     |
[00:56:32]     |
[00:56:32] 429 |             ty::TyRef(_, tam) => match tam.ty.sty {
[00:56:32]     |             ^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:32] 
[00:56:32] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:32]     |
[00:56:32]     |
[00:56:32] 677 |         ty::TyRef(_, ref tm) => walk_ptrs_ty(tm.ty),
[00:56:32]     |         ^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:32] 
[00:56:32] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:32]     |
[00:56:32]     |
[00:56:32] 429 |             ty::TyRef(_, tam) => match tam.ty.sty {
[00:56:32]     |             ^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:32] 
[00:56:32] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:32]     |
[00:56:32]     |
[00:56:32] 687 |             ty::TyRef(_, ref tm) => inner(tm.ty, depth + 1),
[00:56:32]     |             ^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:32] 
[00:56:32] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:32]     |
[00:56:32]     |
[00:56:32] 677 |         ty::TyRef(_, ref tm) => walk_ptrs_ty(tm.ty),
[00:56:32]     |         ^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:32] 
[00:56:32] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:32]     |
[00:56:32]     |
[00:56:32] 687 |             ty::TyRef(_, ref tm) => inner(tm.ty, depth + 1),
[00:56:32]     |             ^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:32] 
[00:56:33] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:33]     |
[00:56:33]     |
[00:56:33] 746 |         ty::TyRef(_, ref subty) => is_slice_like(cx, subty.ty),
[00:56:33]     |         ^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:33] 
[00:56:33] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:33]      |
[00:56:33]      |
[00:56:33] 1368 |                 ty::TyRef(_, ref tam) => match (&pat[0].node, &pat[1].node) {
[00:56:33]      |                 ^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:33] 
[00:56:33] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:33]      |
[00:56:33]      |
[00:56:33] 1708 |                     if let ty::TyRef(_, mutbl) = ty.sty {
[00:56:33]      |                            ^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:33] 
[00:56:33] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:33]      |
[00:56:33]      |
[00:56:33] 1720 |                     if let ty::TyRef(_, mutbl) = ty.sty {
[00:56:33]      |                            ^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:33] 
[00:56:33] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:33]    |
[00:56:33]    |
[00:56:33] 54 |                                     if let ty::TyRef(_, tam) = ty.sty {
[00:56:33]    |                                            ^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:33] 
[00:56:33] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:33]     |
[00:56:33]     |
[00:56:33] 746 |         ty::TyRef(_, ref subty) => is_slice_like(cx, subty.ty),
[00:56:33]     |         ^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:33] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]      |
[00:56:34]      |
[00:56:34] 1368 |                 ty::TyRef(_, ref tam) => match (&pat[0].node, &pat[1].node) {
[00:56:34]      |                 ^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]      |
[00:56:34]      |
[00:56:34] 1708 |                     if let ty::TyRef(_, mutbl) = ty.sty {
[00:56:34]      |                            ^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]      |
[00:56:34]      |
[00:56:34] 1720 |                     if let ty::TyRef(_, mutbl) = ty.sty {
[00:56:34]      |                            ^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 752 |                     ty::TyRef(_, ty) if ty.ty.sty == ty::TyStr => for &(method, pos) in &PATTERN_METHODS {
[00:56:34]     |                     ^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    |
[00:56:34]    |
[00:56:34] 54 |                                     if let ty::TyRef(_, tam) = ty.sty {
[00:56:34]    |                                            ^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 970 |     if let ty::TyRef(_, ty::TypeAndMut { ty: inner, .. }) = arg_ty.sty {
[00:56:34]     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 971 |         if let ty::TyRef(_, ty::TypeAndMut { ty: innermost, .. }) = inner.sty {
[00:56:34]     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 981 |                     while let ty::TyRef(_, ty::TypeAndMut { ty: inner, .. }) = ty.sty {
[00:56:34]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]      |
[00:56:34]      |
[00:56:34] 1318 |             ty::TyRef(_, ty::TypeAndMut { ty: inner, .. }) => if may_slice(cx, inner) {
[00:56:34]      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]      |
[00:56:34]      |
[00:56:34] 1303 |             ty::TyRef(_, ty::TypeAndMut { ty: inner, .. }) => may_slice(cx, inner),
[00:56:34]      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 752 |                     ty::TyRef(_, ty) if ty.ty.sty == ty::TyStr => for &(method, pos) in &PATTERN_METHODS {
[00:56:34]     |                     ^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 970 |     if let ty::TyRef(_, ty::TypeAndMut { ty: inner, .. }) = arg_ty.sty {
[00:56:34]     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 971 |         if let ty::TyRef(_, ty::TypeAndMut { ty: innermost, .. }) = inner.sty {
[00:56:34]     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 981 |                     while let ty::TyRef(_, ty::TypeAndMut { ty: inner, .. }) = ty.sty {
[00:56:34]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    |
[00:56:34]    |
[00:56:34] 73 |               } else if let ty::TyRef(
[00:56:34] 74 | |                 _,
[00:56:34] 74 | |                 _,
[00:56:34] 75 | |                 ty::TypeAndMut {
[00:56:34] 76 | |                     mutbl: hir::MutMutable,
[00:56:34] 78 | |                 },
[00:56:34] 78 | |                 },
[00:56:34] 79 | |             ) = self.cx.tables.expr_ty(e).sty
[00:56:34]    | |_____________^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]      |
[00:56:34]      |
[00:56:34] 1318 |             ty::TyRef(_, ty::TypeAndMut { ty: inner, .. }) => if may_slice(cx, inner) {
[00:56:34]      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]      |
[00:56:34]      |
[00:56:34] 1303 |             ty::TyRef(_, ty::TypeAndMut { ty: inner, .. }) => may_slice(cx, inner),
[00:56:34]      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    |
[00:56:34] 64 | /                     ty::TyRef(
[00:56:34] 65 | |                         _,
[00:56:34] 65 | |                         _,
[00:56:34] 66 | |                         ty::TypeAndMut {
[00:56:34] 67 | |                             mutbl: MutImmutable,
[00:56:34] 69 | |                         },
[00:56:34] 70 | |                     ) |
[00:56:34] 70 | |                     ) |
[00:56:34]    | |_____________________^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    |
[00:56:34]    |
[00:56:34] 80 |             if let ty::TyRef(_, ref tam) = cx.tables.pat_ty(pat).sty;
[00:56:34]    |                    ^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    |
[00:56:34]    |
[00:56:34] 82 |             if let ty::TyRef(_, ref tam) = tam.ty.sty;
[00:56:34]    |                    ^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    |
[00:56:34]    |
[00:56:34] 73 |               } else if let ty::TyRef(
[00:56:34] 74 | |                 _,
[00:56:34] 74 | |                 _,
[00:56:34] 75 | |                 ty::TypeAndMut {
[00:56:34] 76 | |                     mutbl: hir::MutMutable,
[00:56:34] 78 | |                 },
[00:56:34] 78 | |                 },
[00:56:34] 79 | |             ) = self.cx.tables.expr_ty(e).sty
[00:56:34]    | |_____________^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    |
[00:56:34] 64 | /                     ty::TyRef(
[00:56:34] 65 | |                         _,
[00:56:34] 65 | |                         _,
[00:56:34] 66 | |                         ty::TypeAndMut {
[00:56:34] 67 | |                             mutbl: MutImmutable,
[00:56:34] 69 | |                         },
[00:56:34] 70 | |                     ) |
[00:56:34] 70 | |                     ) |
[00:56:34]    | |_____________________^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    |
[00:56:34]    |
[00:56:34] 80 |             if let ty::TyRef(_, ref tam) = cx.tables.pat_ty(pat).sty;
[00:56:34]    |                    ^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    |
[00:56:34]    |
[00:56:34] 82 |             if let ty::TyRef(_, ref tam) = tam.ty.sty;
[00:56:34]    |                    ^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/ptr.rs:153:16
[00:56:34]     |
[00:56:34] 153 |           if let ty::TyRef(
[00:56:34] 154 | |             _,
[00:56:34] 154 | |             _,
[00:56:34] 155 | |             ty::TypeAndMut {
[00:56:34] 156 | |                 ty,
[00:56:34] 157 | |                 mutbl: MutImmutable,
[00:56:34] 158 | |             },
[00:56:34] 159 | |         ) = ty.sty
[00:56:34]     | |_________^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/ptr.rs:153:16
[00:56:34]     |
[00:56:34] 153 |           if let ty::TyRef(
[00:56:34] 154 | |             _,
[00:56:34] 154 | |             _,
[00:56:34] 155 | |             ty::TypeAndMut {
[00:56:34] 156 | |                 ty,
[00:56:34] 157 | |                 mutbl: MutImmutable,
[00:56:34] 158 | |             },
[00:56:34] 159 | |         ) = ty.sty
[00:56:34]     | |_________^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 232 |                             (&ty::TyRef(_, rty), &ty::TyRawPtr(ptr_ty)) => span_lint_and_then(
[00:56:34]     |                               ^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 287 |                             (&ty::TyRawPtr(from_pty), &ty::TyRef(_, to_ref_ty)) => span_lint_and_then(
[00:56:34]     |                                                        ^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 334 |                             (&ty::TyRef(_, ref ref_from), &ty::TyRef(_, ref ref_to)) => {
[00:56:34]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 334 |                             (&ty::TyRef(_, ref ref_from), &ty::TyRef(_, ref ref_to)) => {
[00:56:34]     |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 232 |                             (&ty::TyRef(_, rty), &ty::TyRawPtr(ptr_ty)) => span_lint_and_then(
[00:56:34]     |                               ^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 287 |                             (&ty::TyRawPtr(from_pty), &ty::TyRef(_, to_ref_ty)) => span_lint_and_then(
[00:56:34]     |                                                        ^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 334 |                             (&ty::TyRef(_, ref ref_from), &ty::TyRef(_, ref ref_to)) => {
[00:56:34]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     |
[00:56:34]     |
[00:56:34] 334 |                             (&ty::TyRef(_, ref ref_from), &ty::TyRef(_, ref ref_to)) => {
[00:56:34]     |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    |
[00:56:34]    |
[00:56:34] 38 |             if let ty::TyRef(_, ref ty) = cx.tables.expr_ty_adjusted(expr).sty;
[00:56:34]    |                    ^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:35] error: aborting due to 24 previous errors
[00:56:35] 
[00:56:35] For more information about this error, try `rustc --explain E0023`.
[00:56:35] [RUSTC-TIMING] clippy_lints test:false 6.407
[00:56:35] [RUSTC-TIMING] clippy_lints test:false 6.407
[00:56:35] error: Could not compile `clippy_lints`.
[00:56:35] 
[00:56:35] Caused by:
[00:56:35]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name clippy_lints tools/clippy/clippy_lints/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=83a1334c5fea0e9c -C extra-filename=-83a1334c5fea0e9c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-7f2f9a2d0435c18e.rlib --extern serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-2cf386f66182433d.so --extern quine_mc_cluskey=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquine_mc_cluskey-0e4603a69e92573a.rlib --extern semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsemver-b1deacd60bfb3467.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblazy_static-b9cb06bc45180e7a.rlib --extern url=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liburl-45f65d65dd6d2bb7.rlib --extern if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-4752be648b1a17ac.rlib --extern matches=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libmatches-75ce18cd1348fe32.rlib --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-527d35c40b476be0.rlib --extern toml=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtoml-5c9967c64118fa51.rlib --extern cargo_metadata=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libcargo_metadata-1eb24d7fae38db8d.rlib --extern unicode_normalization=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libunicode_normalization-647d63666653882f.rlib --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-0be9d273ba4062d8.rlib --extern regex_syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex_syntax-201758bda4bd8ffb.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/backtrace-sys-2365887b342ce731/out/.libs` (exit code: 101)
[00:56:35] warning: build failed, waiting for other jobs to finish...
[00:56:35] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:35]    |
[00:56:35]    |
[00:56:35] 38 |             if let ty::TyRef(_, ref ty) = cx.tables.expr_ty_adjusted(expr).sty;
[00:56:35]    |                    ^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:35] error: aborting due to 24 previous errors
[00:56:35] 
[00:56:35] For more information about this error, try `rustc --explain E0023`.
[00:56:35] error: Could not compile `clippy_lints`.
---
Building stage2 tool miri (x86_64-unknown-linux-gnu)
[01:17:26]    Compiling miri v0.1.0 (file:///checkout/src/tools/miri)
[01:17:26]    Compiling byteorder v1.2.2
[01:17:27] [RUSTC-TIMING] byteorder test:false 0.827
[01:17:30] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[01:17:30]     |
[01:17:30]     |
[01:17:30] 404 |             ty::TyRef(_, ty::TypeAndMut { ty: pointee, .. }) |
[01:17:30]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[01:17:30] 
[01:17:30] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[01:17:30]     |
[01:17:30]     |
[01:17:30] 661 | /                 TyRef(region,
[01:17:30] 662 | |                     ty::TypeAndMut {
[01:17:30] 663 | |                         ty: pointee_ty,
[01:17:30] 664 | |                         mutbl,
[01:17:30] 665 | |                     }) => {
[01:17:30]     | |______________________^ expected 3 fields, found 2
[01:17:30] error: aborting due to 2 previous errors
[01:17:30] 
[01:17:30] For more information about this error, try `rustc --explain E0023`.
[01:17:30] [RUSTC-TIMING] miri test:false 3.137
---
[01:17:30] Cloning into 'rust-toolstate'...
[01:17:30] [master 7eff5f2] (linux CI update)
[01:17:30]  1 file changed, 1 insertion(+)
[01:17:34] To https://github.com/rust-lang-nursery/rust-toolstate.git
[01:17:34]    eec401a..7eff5f2  master -> master
[01:17:34] Error: The state of "clippy-driver" has regressed from "test-pass" to "build-fail"
[01:17:34] Error: The state of "miri" has regressed from "test-pass" to "build-fail"

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0534c600
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
