plain
failures:

---- [ui] tests/ui/imports/issue-59764.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/imports/issue-59764.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-59764" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-59764/auxiliary" "--extern" "issue_59764" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:14:33
   |
LL |     use issue_59764::foo::{baz, makro};
   |                                 ^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL -     use issue_59764::foo::{baz, makro};
LL +     use issue_59764::{makro, foo::{baz}};


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:21:9
   |
LL |         makro, //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]
   |         ^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL ~     use issue_59764::{makro, foo::{
LL |         baz,
LL ~         //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]
LL ~     }};


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:28:9
   |
LL |         makro //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]
   |         ^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL ~     use issue_59764::{makro, foo::{
LL |         baz,
LL ~         //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]
LL ~     }};


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:33:33
   |
LL |     use issue_59764::foo::{baz, makro, foobar};
   |                                 ^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL -     use issue_59764::foo::{baz, makro, foobar};
LL +     use issue_59764::{makro, foo::{baz, foobar}};


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:40:9
   |
LL |         makro, //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]
   |         ^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL ~     use issue_59764::{makro, foo::{
LL |         baz,
LL ~         //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]
LL |         foobar,
LL ~     }};


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:48:9
   |
LL |         makro, //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]
   |         ^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL ~     use issue_59764::{makro, foo::{
LL |         baz,
LL ~         //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]
LL |         foobar
LL ~     }};


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:54:31
   |
LL |     use issue_59764::{foobaz, foo::makro};
   |                               ^^^^^^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL -     use issue_59764::{foobaz, foo::makro};
LL +     use issue_59764::{makro, foobaz};


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:59:42
   |
LL |     use issue_59764::{foobaz, foo::{baz, makro}};
   |                                          ^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL -     use issue_59764::{foobaz, foo::{baz, makro}};
LL +     use issue_59764::{makro, foobaz, foo::{baz}};


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:68:13
   |
LL |             makro, //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]
   |             ^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL ~     use issue_59764::{makro, 
LL |         foobaz,
LL |         foo::{
LL |             baz,
LL ~             //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:78:13
   |
LL |             makro //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]
   |             ^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL ~     use issue_59764::{makro, 
LL |         foobaz,
LL |         foo::{
LL |             baz,
LL ~             //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:84:42
   |
LL |     use issue_59764::{foobaz, foo::{baz, makro, barbaz::{barfoo}}};
   |                                          ^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL -     use issue_59764::{foobaz, foo::{baz, makro, barbaz::{barfoo}}};
LL +     use issue_59764::{makro, foobaz, foo::{baz, barbaz::{barfoo}}};


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:93:13
   |
LL |             makro, //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]
   |             ^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL ~     use issue_59764::{makro, 
LL |         foobaz,
LL |         foo::{
LL |             baz,
LL ~             //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:102:9
   |
LL |     use issue_59764::foo::makro as baz;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL |     use issue_59764::makro as baz;


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:107:33
   |
LL |     use issue_59764::foo::{baz, makro as foobar};
   |                                 ^^^^^^^^^^^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL -     use issue_59764::foo::{baz, makro as foobar};
LL +     use issue_59764::{makro as foobar, foo::{baz}};


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:120:17
   |
LL |                 makro as foobar} //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]
   |                 ^^^^^^^^^^^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL ~         issue_59764::{makro as foobar, 
 ...
LL | 
LL | 
LL ~             foo::{baz} //~ ERROR unresolved import `issue_59764::foo::makro` [E0432]


error[E0432]: unresolved import `issue_59764::foo::makro`
  --> fake-test-src-base/imports/issue-59764.rs:127:5
   |
LL | use issue_59764::foo::makro;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ no `makro` in `foo`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
LL | use issue_59764::makro;


error: cannot determine resolution for the macro `makro`
  --> fake-test-src-base/imports/issue-59764.rs:130:1
   |
LL | makro!(bar);
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
thread 'rustc' panicked at 'attempt to subtract with overflow', compiler/rustc_resolve/src/check_unused.rs:288:20
stack backtrace:
   0:     0x7fde7927f444 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbe381ee9d8fb80ac
   1:     0x7fde792e68d8 - core::fmt::write::h6baae6d908c8aa29
   1:     0x7fde792e68d8 - core::fmt::write::h6baae6d908c8aa29
   2:     0x7fde79273b71 - std::io::Write::write_fmt::he036d21813e1d37b
   3:     0x7fde7927f251 - std::sys_common::backtrace::print::h6bbad86a50a9b807
   4:     0x7fde7928231a - std::panicking::default_hook::{{closure}}::h89ce5225b1abbcd0
   5:     0x7fde79281ffc - std::panicking::default_hook::h2076b580de7da8aa
   6:     0x7fde79d42735 - rustc_driver_impl[35a922b630090adc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fde79282a27 - std::panicking::rust_panic_with_hook::h96f142fc327b9102
   8:     0x7fde79282769 - std::panicking::begin_panic_handler::{{closure}}::h1ae1f798f2b68aa6
   9:     0x7fde7927f926 - std::sys_common::backtrace::__rust_end_short_backtrace::he78842854e47691f
  11:     0x7fde792370c3 - core::panicking::panic_fmt::hc112adcd3048e87a
  11:     0x7fde792370c3 - core::panicking::panic_fmt::hc112adcd3048e87a
  12:     0x7fde79237153 - core::panicking::panic::h31d3eb6ac2a0110a
  13:     0x7fde7add24da - rustc_resolve[12c51da166f76470]::check_unused::calc_unused_spans
  14:     0x7fde7adfc1e5 - <rustc_resolve[12c51da166f76470]::Resolver>::check_unused
  15:     0x7fde7add691f - <rustc_session[df42b5c8423ee0c6]::session::Session>::time::<(), <rustc_resolve[12c51da166f76470]::Resolver>::resolve_crate::{closure#0}>
  16:     0x7fde7ae2938d - <rustc_resolve[12c51da166f76470]::Resolver>::resolve_crate
  17:     0x7fde79e2ba6f - rustc_interface[40fabea1090bc725]::passes::resolver_for_lowering
  18:     0x7fde7baa6342 - rustc_query_system[30248b13c9348ba2]::query::plumbing::try_execute_query::<rustc_query_impl[24f4096b3b10d9a8]::queries::resolver_for_lowering, rustc_query_impl[24f4096b3b10d9a8]::plumbing::QueryCtxt>
  19:     0x7fde7b89a21b - rustc_query_impl[24f4096b3b10d9a8]::get_query::resolver_for_lowering
  20:     0x7fde79d5f5ea - <rustc_middle[e04a56042143a964]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[35a922b630090adc]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[33de10c28208409b]::steal::Steal<(rustc_middle[e04a56042143a964]::ty::ResolverAstLowering, alloc[5dd3737e47d928e9]::rc::Rc<rustc_ast[859c71a87bb94b48]::ast::Crate>)>>
  21:     0x7fde79d8dd50 - <rustc_interface[40fabea1090bc725]::interface::Compiler>::enter::<rustc_driver_impl[35a922b630090adc]::run_compiler::{closure#1}::{closure#2}, core[903adaea1e5a9fd0]::result::Result<core[903adaea1e5a9fd0]::option::Option<rustc_interface[40fabea1090bc725]::queries::Linker>, rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>>
  22:     0x7fde79d588b0 - rustc_span[ee0d9697df34b4e8]::set_source_map::<core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>, rustc_interface[40fabea1090bc725]::interface::run_compiler<core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>, rustc_driver_impl[35a922b630090adc]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  23:     0x7fde79d4ff89 - <scoped_tls[8e348592f5c050ed]::ScopedKey<rustc_span[ee0d9697df34b4e8]::SessionGlobals>>::set::<rustc_interface[40fabea1090bc725]::interface::run_compiler<core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>, rustc_driver_impl[35a922b630090adc]::run_compiler::{closure#1}>::{closure#0}, core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>>
  24:     0x7fde79d68516 - std[e1542e44bb495e20]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[40fabea1090bc725]::util::run_in_thread_pool_with_globals<rustc_interface[40fabea1090bc725]::interface::run_compiler<core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>, rustc_driver_impl[35a922b630090adc]::run_compiler::{closure#1}>::{closure#0}, core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>>
  25:     0x7fde79da8648 - std[e1542e44bb495e20]::panicking::try::<core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>, core[903adaea1e5a9fd0]::panic::unwind_safe::AssertUnwindSafe<<std[e1542e44bb495e20]::thread::Builder>::spawn_unchecked_<rustc_interface[40fabea1090bc725]::util::run_in_thread_pool_with_globals<rustc_interface[40fabea1090bc725]::interface::run_compiler<core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>, rustc_driver_impl[35a922b630090adc]::run_compiler::{closure#1}>::{closure#0}, core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  26:     0x7fde79d5610d - <<std[e1542e44bb495e20]::thread::Builder>::spawn_unchecked_<rustc_interface[40fabea1090bc725]::util::run_in_thread_pool_with_globals<rustc_interface[40fabea1090bc725]::interface::run_compiler<core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>, rustc_driver_impl[35a922b630090adc]::run_compiler::{closure#1}>::{closure#0}, core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[903adaea1e5a9fd0]::result::Result<(), rustc_span[ee0d9697df34b4e8]::ErrorGuaranteed>>::{closure#1} as core[903adaea1e5a9fd0]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7fde7928f3de - std::sys::unix::thread::Thread::new::thread_start::h7b454d9008baa60f
  28:     0x7fde79027b43 - <unknown>
  29:     0x7fde790b9a00 - <unknown>
  30:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (bf7524213 2023-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
error: aborting due to 17 previous errors
