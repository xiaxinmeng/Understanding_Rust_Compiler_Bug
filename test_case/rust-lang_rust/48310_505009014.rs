`
   Compiling f v0.1.0 (/tmp/f)
warning: trait objects without an explicit `dyn` are deprecated
 --> src/lib.rs:7:21
  |
7 |         let f: &mut fmt::Write = f;
  |                     ^^^^^^^^^^ help: use `dyn`: `dyn fmt::Write`
  |
  = note: #[warn(bare_trait_objects)] on by default

Unusual: noalias argument aliases another argument
  call void @_ZN4core3fmt9Arguments6new_v117hf2b276cf93a5bc30E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %3, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 bitcast (<{ [0 x i8] }>* @1 to [0 x { [0 x i8]*, i64 }]*), i64 0, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 bitcast (<{ [0 x i8] }>* @1 to [0 x { i8*, i8* }]*), i64 0), !dbg !124
Undefined behavior: Call argument type mismatches callee parameter type
  %6 = call zeroext i1 bitcast (i1 (%"core::fmt::Formatter"*, %"core::fmt::Arguments"*)* @"_ZN57_$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$9write_fmt17h96fcf612e8ddc9f1E" to i1 ({}*, %"core::fmt::Arguments"*)*)({}* align 1 %4, %"core::fmt::Arguments"* noalias nocapture dereferenceable(48) %3), !dbg !124
    Finished release [optimized + debuginfo] target(s) in 0.68s
