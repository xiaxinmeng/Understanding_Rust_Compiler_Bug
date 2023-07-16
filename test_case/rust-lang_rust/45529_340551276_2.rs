
$ rg FOO *.ll
foo.foo1-8787f43e282added376259c1adb08b80.rs.rust-cgu.ll:@_ZN3foo3FOO17h556917dcc656cbe0E.llvm.54FD3A5C = hidden thread_local global i32 3, align 4
foo.foo1-8787f43e282added376259c1adb08b80.rs.rust-cgu.ll:  ret i32* @_ZN3foo3FOO17h556917dcc656cbe0E.llvm.54FD3A5C
foo.foo0-8787f43e282added376259c1adb08b80.rs.rust-cgu.ll:@_ZN3foo3FOO17h556917dcc656cbe0E.llvm.54FD3A5C = external hidden thread_local local_unnamed_addr global i32, align 4
foo.foo0-8787f43e282added376259c1adb08b80.rs.rust-cgu.ll: %0 = load i32, i32* @_ZN3foo3FOO17h556917dcc656cbe0E.llvm.54FD3A5C, align 4
