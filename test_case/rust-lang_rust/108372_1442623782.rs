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
    
    --- stdout
    
    running 1 test
    test /tmp/mdbook-0x1wTc/generics/assoc_items/the_problem.md - The_Problem (line 15) ... FAILED
    failures:
    
    
    ---- /tmp/mdbook-0x1wTc/generics/assoc_items/the_problem.md - The_Problem (line 15) stdout ----
    error: binary operation on reference to `Copy` type `i32`
      --> /tmp/mdbook-0x1wTc/generics/assoc_items/the_problem.md:29:18
       |
    15 |         (&self.0 == number_1) && (&self.1 == number_2)
       |
       |
       = note: `i32` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
       = note: `#[deny(ref_binop_on_copy_type)]` on by default
    help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
       |
    15 -         (&self.0 == number_1) && (&self.1 == number_2)
    15 +         (self.0 == *number_1) && (&self.1 == number_2)
    
    error: binary operation on reference to `Copy` type `i32`
    error: binary operation on reference to `Copy` type `i32`
      --> /tmp/mdbook-0x1wTc/generics/assoc_items/the_problem.md:29:43
       |
    15 |         (&self.0 == number_1) && (&self.1 == number_2)
       |
       |
       = note: `i32` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
    help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
       |
    15 -         (&self.0 == number_1) && (&self.1 == number_2)
    15 +         (&self.0 == number_1) && (self.1 == *number_2)
    
    error: aborting due to 2 previous errors
    
    Couldn't compile the test.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-0x1wTc/generics/assoc_items/the_problem.md - The_Problem (line 15)
    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
    
    
    --- stderr
    --- stderr
    
[2023-02-24T00:26:06Z ERROR mdbook::book] rustdoc returned an error:
    
    --- stdout
    
    running 3 tests
    test /tmp/mdbook-0x1wTc/generics/assoc_items/types.md - Associated_types (line 23) ... ignored
    test /tmp/mdbook-0x1wTc/generics/assoc_items/types.md - Associated_types (line 34) ... FAILED
    test /tmp/mdbook-0x1wTc/generics/assoc_items/types.md - Associated_types (line 7) ... ok
    failures:
    
    
    ---- /tmp/mdbook-0x1wTc/generics/assoc_items/types.md - Associated_types (line 34) stdout ----
    error: binary operation on reference to `Copy` type `i32`
      --> /tmp/mdbook-0x1wTc/generics/assoc_items/types.md:58:18
       |
    25 |         (&self.0 == number_1) && (&self.1 == number_2)
       |
       |
       = note: `i32` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
       = note: `#[deny(ref_binop_on_copy_type)]` on by default
    help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
       |
    25 -         (&self.0 == number_1) && (&self.1 == number_2)
    25 +         (self.0 == *number_1) && (&self.1 == number_2)
    
    error: binary operation on reference to `Copy` type `i32`
    error: binary operation on reference to `Copy` type `i32`
      --> /tmp/mdbook-0x1wTc/generics/assoc_items/types.md:58:43
       |
    25 |         (&self.0 == number_1) && (&self.1 == number_2)
       |
       |
       = note: `i32` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
    help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
       |
    25 -         (&self.0 == number_1) && (&self.1 == number_2)
    25 +         (&self.0 == number_1) && (self.1 == *number_2)
    
    error: aborting due to 2 previous errors
    
    Couldn't compile the test.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-0x1wTc/generics/assoc_items/types.md - Associated_types (line 34)
    test result: FAILED. 1 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.17s
    
    
    --- stderr
    --- stderr
    
[2023-02-24T00:26:12Z ERROR mdbook::book] rustdoc returned an error:
    
    --- stdout
    
    running 5 tests
    test /tmp/mdbook-0x1wTc/trait/impl_trait.md - _can_be_used_in_two_locations_::As_a_return_type (line 115) ... FAILED
    test /tmp/mdbook-0x1wTc/trait/impl_trait.md - _can_be_used_in_two_locations_::As_an_argument_type (line 14) ... ok
    test /tmp/mdbook-0x1wTc/trait/impl_trait.md - _can_be_used_in_two_locations_::As_an_argument_type (line 33) ... ok
    test /tmp/mdbook-0x1wTc/trait/impl_trait.md - _can_be_used_in_two_locations_::As_a_return_type (line 97) ... ok
    test /tmp/mdbook-0x1wTc/trait/impl_trait.md - _can_be_used_in_two_locations_::As_a_return_type (line 57) ... ok
    failures:
    
    
    ---- /tmp/mdbook-0x1wTc/trait/impl_trait.md - _can_be_used_in_two_locations_::As_a_return_type (line 115) stdout ----
    error: binary operation on reference to `Copy` type `i32`
     --> /tmp/mdbook-0x1wTc/trait/impl_trait.md:119:23
      |
    5 |         .filter(|x| x > &&0)
      |
      |
      = note: `i32` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
      = note: `#[deny(ref_binop_on_copy_type)]` on by default
    help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
      |
    5 -         .filter(|x| x > &&0)
    5 +         .filter(|x| *x > 0)
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-0x1wTc/trait/impl_trait.md - _can_be_used_in_two_locations_::As_a_return_type (line 115)
    test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
    
    
    --- stderr
---
    
    --- stdout
    
    running 3 tests
    test /tmp/mdbook-pLIz4p/ch10-00-generics.md - Generic_Types__Traits__and_Lifetimes::Removing_Duplication_by_Extracting_a_Function (line 129) ... FAILED
    test /tmp/mdbook-pLIz4p/ch10-00-generics.md - Generic_Types__Traits__and_Lifetimes::Removing_Duplication_by_Extracting_a_Function (line 82) ... FAILED
    test /tmp/mdbook-pLIz4p/ch10-00-generics.md - Generic_Types__Traits__and_Lifetimes::Removing_Duplication_by_Extracting_a_Function (line 47) ... FAILED
    failures:
    
    
    ---- /tmp/mdbook-pLIz4p/ch10-00-generics.md - Generic_Types__Traits__and_Lifetimes::Removing_Duplication_by_Extracting_a_Function (line 129) stdout ----
    error: binary operation on reference to `Copy` type `i32`
     --> /tmp/mdbook-pLIz4p/ch10-00-generics.md:134:17
    6 |         if item > largest {
      |                 ^
      |
      |
      = note: `i32` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
      = note: `#[deny(ref_binop_on_copy_type)]` on by default
    help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
      |
    6 |         if *item > *largest {
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    Couldn't compile the test.
    ---- /tmp/mdbook-pLIz4p/ch10-00-generics.md - Generic_Types__Traits__and_Lifetimes::Removing_Duplication_by_Extracting_a_Function (line 82) stdout ----
    error: binary operation on reference to `Copy` type `i32`
     --> /tmp/mdbook-pLIz4p/ch10-00-generics.md:89:19
      |
    8 |         if number > largest {
      |
      |
      = note: `i32` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
      = note: `#[deny(ref_binop_on_copy_type)]` on by default
    help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
      |
    8 |         if *number > *largest {
    
    error: binary operation on reference to `Copy` type `i32`
    error: binary operation on reference to `Copy` type `i32`
      --> /tmp/mdbook-pLIz4p/ch10-00-generics.md:101:19
       |
    20 |         if number > largest {
       |
       |
       = note: `i32` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
    help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
       |
    20 |         if *number > *largest {
    
    error: aborting due to 2 previous errors
    
    Couldn't compile the test.
    Couldn't compile the test.
    ---- /tmp/mdbook-pLIz4p/ch10-00-generics.md - Generic_Types__Traits__and_Lifetimes::Removing_Duplication_by_Extracting_a_Function (line 47) stdout ----
    error: binary operation on reference to `Copy` type `i32`
     --> /tmp/mdbook-pLIz4p/ch10-00-generics.md:54:19
      |
    8 |         if number > largest {
      |
      |
      = note: `i32` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
      = note: `#[deny(ref_binop_on_copy_type)]` on by default
    help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
      |
    8 |         if *number > *largest {
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-pLIz4p/ch10-00-generics.md - Generic_Types__Traits__and_Lifetimes::Removing_Duplication_by_Extracting_a_Function (line 129)
        /tmp/mdbook-pLIz4p/ch10-00-generics.md - Generic_Types__Traits__and_Lifetimes::Removing_Duplication_by_Extracting_a_Function (line 47)
        /tmp/mdbook-pLIz4p/ch10-00-generics.md - Generic_Types__Traits__and_Lifetimes::Removing_Duplication_by_Extracting_a_Function (line 82)
    test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
    
    
    --- stderr
    --- stderr
    
[2023-02-24T00:26:32Z ERROR mdbook::book] rustdoc returned an error:
    
    --- stdout
    
    running 13 tests
    test /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::In_Function_Definitions (line 83) ... ignored
    test /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::In_Function_Definitions (line 99) ... ignored
    test /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::In_Struct_Definitions (line 197) ... ignored
    test /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::In_Function_Definitions (line 21) ... FAILED
    test /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::In_Struct_Definitions (line 169) ... ok
    test /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::In_Enum_Definitions (line 281) ... ok
    test /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::Performance_of_Code_Using_Generics (line 455) ... ok
    test /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::In_Enum_Definitions (line 264) ... ok
    test /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::Performance_of_Code_Using_Generics (line 472) ... ok
    test /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::In_Struct_Definitions (line 236) ... ok
    test /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::In_Method_Definitions (line 352) ... ok
    test /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::In_Method_Definitions (line 395) ... ok
    test /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::In_Method_Definitions (line 309) ... ok
    failures:
    
    
    ---- /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::In_Function_Definitions (line 21) stdout ----
    error: binary operation on reference to `Copy` type `i32`
     --> /tmp/mdbook-pLIz4p/ch10-01-syntax.md:26:17
    6 |         if item > largest {
      |                 ^
      |
      |
      = note: `i32` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
      = note: `#[deny(ref_binop_on_copy_type)]` on by default
    help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
      |
    6 |         if *item > *largest {
    
    error: binary operation on reference to `Copy` type `char`
    error: binary operation on reference to `Copy` type `char`
      --> /tmp/mdbook-pLIz4p/ch10-01-syntax.md:38:17
    18 |         if item > largest {
       |                 ^
       |
       |
       = note: `char` takes `4` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
    help: dereferencing the expressions will allow the compiler to more consistently optimize these binary operations
       |
    18 |         if *item > *largest {
    
    error: aborting due to 2 previous errors
    
    Couldn't compile the test.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-pLIz4p/ch10-01-syntax.md - _::Generic_Data_Types::In_Function_Definitions (line 21)
    test result: FAILED. 9 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.20s
    
    
    --- stderr
---

2 command(s) did not execute successfully:
 finished in 2.354 seconds

  - LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin:/node-v14.20.0-linux-x64/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" RUSTC_BOOTSTRAP="1" RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "test" "/checkout/src/doc/rust-by-example"

  - LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin:/node-v14.20.0-linux-x64/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" RUSTC_BOOTSTRAP="1" RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "test" "/checkout/src/doc/book"
Build completed unsuccessfully in 0:11:31
{"embedded-book":"test-pass","book":"test-fail","rustbook":"test-fail","reference":"test-pass","nomicon":"test-pass","edition-guide":"test-pass","rust-by-example":"test-fail"}Building bootstrap
    Finished dev [unoptimized] target(s) in 0.04s
Verifying status of book...
---
   Compiling clippy_utils v0.1.69 (/checkout/src/tools/clippy/clippy_utils)
error: binary operation on reference to `Copy` type `rustc_ast::BindingAnnotation`
  --> src/tools/clippy/clippy_utils/src/ast_utils.rs:40:54
   |
40 |         (Ident(b1, i1, s1), Ident(b2, i2, s2)) => b1 == b2 && eq_id(*i1, *i2) && both(s1, s2, |l, r| eq_pat(l, r)),
   |
   |
   = note: `rustc_ast::BindingAnnotation` takes `2` bytes of memory; copying the value instead of referencing it might avoid unnecessary pointer indirections
   = note: `#[deny(ref_binop_on_copy_type)]` on by default
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
Build completed unsuccessfully in 0:00:10
