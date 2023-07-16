
PS Q:\projects\hello_world> cargo build --verbose
   Compiling advapi32-sys v0.1.2
     Running `rustc C:\Users\<name>\.cargo\registry\src\github.com-0a35038f75765ae4\advapi32-sys-0.1.2\build.rs --crat
e-name build_script_build --crate-type bin -C prefer-dynamic -g --out-dir Q:\projects\hello_world\target\debug\build\adv
api32-sys-cfef7a1f30f1e5f6 --emit=dep-info,link -L dependency=Q:\projects\hello_world\target\debug\deps -L dependency=Q:
\projects\hello_world\target\debug\deps --extern build=Q:\projects\hello_world\target\debug\deps\libbuild-304afb6bdff23d
72.rlib -Awarnings`
       Fresh winapi v0.2.2
       Fresh libc v0.1.10
       Fresh winapi-build v0.1.1
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 5, message: "Access is
 denied.\r\n" } }', ../src/libcore\result.rs:731

stack backtrace:
   1:         0x66139105 - sys::backtrace::write::h2345dffb6069c92cLAs
   2:         0x661428ac - rt::unwind::register::heb8429a612e0fecdqfw
   3:         0x6610561f - rt::unwind::begin_unwind_inner::h3f101c489f219c5fzcw
   4:         0x66105f9a - rt::unwind::begin_unwind_fmt::he6ac4cfedf51005fFbw
   5:         0x661422c3 - rust_begin_unwind
   6:         0x6615ecd9 - panicking::panic_fmt::h99704e91012c930eH8B
   7:         0x65bad8dd - metadata::csearch::is_default_impl::h00b99fbd2aeff47fbQo
   8:         0x65ba2bd9 - metadata::loader::Context<'a>::load_library_crate::h257060908c1ee1f7UTo
   9:         0x65b9ef63 - metadata::cstore::CStore::add_used_link_args::h94dd1e859ee72f80Gko
  10:         0x65b9b097 - metadata::creader::CrateReader<'a>.visit..Visitor<'v>::visit_item::hed65f5c686d0cab949m
  11:         0x65b9e35f - metadata::creader::CrateReader<'a>::read_crates::ha65368decfa753bblon
  12:         0x6ef9eb3b - driver::assign_node_ids_and_map::h05e579775eaf79194Da
  13:         0x6ef82a66 - driver::compile_input::hd2fb89b180675596Tba
  14:         0x6f06460d - run_compiler::h282bf3c3999ca505x7b
  15:         0x6f062295 - run::h1c01070ec49f4ed0d7b
  16:         0x6f061bd9 - run::h1c01070ec49f4ed0d7b
  17:         0x6617d82c - rust_try
  18:         0x6617d809 - rust_try
  19:         0x6612d4d5 - rt::unwind::try::inner_try::h75b7bddb15a6c10cs8v
  20:         0x6f061d97 - run::h1c01070ec49f4ed0d7b
  21:         0x66140404 - sys::process::Command::cwd::heb9c93936e48371awUu
  22:         0x77a759cd - BaseThreadInitThunk

Could not compile `advapi32-sys`.

Caused by:
  Process didn't exit successfully: `rustc C:\Users\<name>\.cargo\registry\src\github.com-0a35038f75765ae4\advapi32-sy
s-0.1.2\build.rs --crate-name build_script_build --crate-type bin -C prefer-dynamic -g --out-dir Q:\projects\hello_world
\target\debug\build\advapi32-sys-cfef7a1f30f1e5f6 --emit=dep-info,link -L dependency=Q:\projects\hello_world\target\debu
g\deps -L dependency=Q:\projects\hello_world\target\debug\deps --extern build=Q:\projects\hello_world\target\debug\deps\
libbuild-304afb6bdff23d72.rlib -Awarnings` (exit code: 101)
