
$ cat foo.rs
#![feature(linkage)]

pub mod foo {
    #[linkage = "weak"] // <- it works without this
    #[no_mangle]
    pub extern "C" fn FOO() -> i32 {
        0
    }
}

mod bar {
    extern "C" {
        fn FOO() -> i32;
    }

    pub fn bar() -> i32 {
        unsafe { FOO() }
    }
}

fn main() {
    bar::bar();
}
$ rustc +nightly foo.rs -O
$ rustc +nightly foo.rs -O -C codegen-units=8
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/alex/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "foo.foo0-317d481089b8c8fe83113de504472633.rs.rcgu.o" "foo.foo1-317d481089b8c8fe83113de504472633.rs.rcgu.o" "foo.foo2-317d481089b8c8fe83113de504472633.rs.rcgu.o" "-o" "foo" "foo.crate.allocator.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-Wl,-O1" "-nodefaultlibs" "-L" "/home/alex/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/home/alex/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-c0e00a209103bd4d.rlib" "/home/alex/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-d1820c7fa19979f2.rlib" "/home/alex/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-7c5a3388e5631edc.rlib" "/home/alex/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-da40b64aab02e5af.rlib" "/home/alex/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-c0187bacb4a8469f.rlib" "/home/alex/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-5ddf3ebd487fc468.rlib" "/home/alex/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-ccc7a59de56dc2df.rlib" "/home/alex/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_unicode-4e8d8bf9e4eb1cb3.rlib" "/home/alex/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a9c6d56995f719c6.rlib" "/home/alex/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-2280e0ce1fded772.rlib" "-Wl,-Bdynamic" "-l" "dl" "-l" "rt" "-l" "pthread" "-l" "pthread" "-l" "gcc_s" "-l" "c" "-l" "m" "-l" "rt" "-l" "pthread" "-l" "util" "-l" "util"
  = note: foo.foo2-317d481089b8c8fe83113de504472633.rs.rcgu.o:foo2-317d481089b8c8fe83113de504472633.rs:function foo::main::hdc141899c6b702c7: error: undefined reference to 'FOO'
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error
