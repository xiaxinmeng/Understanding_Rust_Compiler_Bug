
$ echo '#![feature(wasm_abi)] #[repr(C)] pub union Foo { b: [u8; 9] } #[no_mangle] pub extern "wasm" fn foo(a: Foo) {}' | rustc +nightly --target wasm32-unknown-unknown --crate-type lib --emit asm=/dev/stdout,llvm-ir=/dev/stdout -
[...]
.globl  foo
        .type   foo,@function
foo:
        .functype       foo (i32, i32, i32, i32, i32, i32, i32, i32, i32) -> ()
        end_function
[...]
%Foo = type { [9 x i8] }

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define dso_local void @foo(%Foo %0) unnamed_addr #0 {
start:
  ret void
}
[...]
