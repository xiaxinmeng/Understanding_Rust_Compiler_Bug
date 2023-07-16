
Jamess-MacBook-Pro:catnip jlogan$ RUST_BACKTRACE=full cargo check --example packet
warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/lib.rs:6:12
  |
6 | #![feature(generic_const_exprs)]
  |            ^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information

warning: `catnip` (lib) generated 1 warning
    Checking catnip v0.1.1 (/Users/jlogan/git/catnip-f7ff6dc5fb3ac36f3c06e2cb81f842b0b34e3eed/catnip)
warning: unused import: `EthernetPacket`
 --> examples/packet.rs:3:51
  |
3 | use catnip::enet::{EthernetHeader, EthernetFrame, EthernetPacket, EtherType};
  |                                                   ^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `enetframe`
  --> examples/packet.rs:55:9
   |
55 |     let enetframe: EthernetFrame<UDPPacket<0, 2>, P> = EthernetFrame {
   |         ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_enetframe`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: Error finalizing incremental compilation session directory `/Users/jlogan/git/catnip-f7ff6dc5fb3ac36f3c06e2cb81f842b0b34e3eed/catnip/target/debug/incremental/packet-aiij2leu8w3u/s-g7fesapbqo-1k1ehmu-working`: No such file or directory (os error 2)

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<catnip::udp::UDPPacket<0_usize, 2_usize> as catnip::Transportable<36_usize>>, [])` during codegen
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/codegen.rs:69:32

error: internal compiler error: ty::ConstKind::Error constructed but no error reported
  |
  = note: delayed at /rustc/d3ad51b48f83329fac0cd8a9f1253f3146613c1c/compiler/rustc_middle/src/ty/consts.rs:267:43

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/coercion.rs:158:49

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:869:27

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/coercion.rs:1348:42

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_infer/src/infer/sub.rs:121:31

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/fallback.rs:120:58

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at /rustc/d3ad51b48f83329fac0cd8a9f1253f3146613c1c/compiler/rustc_middle/src/ty/relate.rs:390:59

error: internal compiler error: cat_expr Errd
  --> examples/packet.rs:9:17
   |
9  |   fn main() -> () {
   |  _________________^
10 | |     // Some made-up addresses
11 | |     // MAC address in locally-administered address range
12 | |     // IP addresses in local network range
...  |
58 | |     };
59 | | }
   | |_^
   |
   = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31

error: internal compiler error: cat_expr Errd
  --> examples/packet.rs:55:56
   |
55 |       let enetframe: EthernetFrame<UDPPacket<0, 2>, P> = EthernetFrame {
   |  ________________________________________________________^
56 | |         header: enetheader,
57 | |         data: udppacket
58 | |     };
   | |_____^
   |
   = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:721:18

error: internal compiler error: PromoteTemps: MIR had errors
 --> examples/packet.rs:9:1
  |
9 | fn main() -> () {
  | ^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:53:22

error: internal compiler error: broken MIR in DefId(0:44 ~ packet[1334]::main) ("return type"): bad type [type error]
 --> examples/packet.rs:9:1
  |
9 | fn main() -> () {
  | ^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:541:13

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:796:20

error: internal compiler error: broken MIR in DefId(0:44 ~ packet[1334]::main) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: examples/packet.rs:9:1: 9:16 (#0), scope: scope[0] } }): bad type [type error]
 --> examples/packet.rs:9:1
  |
9 | fn main() -> () {
  | ^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:541:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1188:13
stack backtrace:
   0:        0x10e73f401 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h76fb20920f6a83cf
   1:        0x10e79f35b - core::fmt::write::h862e4b55ad2069b4
   2:        0x10e7304ee - std::io::Write::write_fmt::h8083c2fcbfe2364d
   3:        0x10e743770 - std::panicking::default_hook::{{closure}}::h66a9173bcb465cf5
   4:        0x10e743454 - std::panicking::default_hook::hb2e31fbd015eb3d1
   5:        0x117eea80a - rustc_driver[55b90faad7aa1d7b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:        0x10e74403c - std::panicking::rust_panic_with_hook::h87ab016847b73da9
   7:        0x11c4e2547 - std[7522013a281df534]::panicking::begin_panic::<rustc_errors[1ec2a23e13b589f5]::ExplicitBug>::{closure#0}
   8:        0x11c4e24f9 - std[7522013a281df534]::sys_common::backtrace::__rust_end_short_backtrace::<std[7522013a281df534]::panicking::begin_panic<rustc_errors[1ec2a23e13b589f5]::ExplicitBug>::{closure#0}, !>
   9:        0x11c821191 - std[7522013a281df534]::panicking::begin_panic::<rustc_errors[1ec2a23e13b589f5]::ExplicitBug>
  10:        0x11c518329 - std[7522013a281df534]::panic::panic_any::<rustc_errors[1ec2a23e13b589f5]::ExplicitBug>
  11:        0x11c5120aa - <rustc_errors[1ec2a23e13b589f5]::HandlerInner as core[c951771928f8d9c9]::ops::drop::Drop>::drop
  12:        0x117f01d8a - core[c951771928f8d9c9]::ptr::drop_in_place::<rustc_session[75ebba5516f37af0]::parse::ParseSess>
  13:        0x117f0a537 - <alloc[57a39d941921ee5e]::rc::Rc<rustc_session[75ebba5516f37af0]::session::Session> as core[c951771928f8d9c9]::ops::drop::Drop>::drop
  14:        0x117ea260f - core[c951771928f8d9c9]::ptr::drop_in_place::<rustc_interface[112a8d2e82d7db12]::interface::Compiler>
  15:        0x117ea9bec - rustc_span[be5d9d9c22212131]::with_source_map::<core[c951771928f8d9c9]::result::Result<(), rustc_errors[1ec2a23e13b589f5]::ErrorReported>, rustc_interface[112a8d2e82d7db12]::interface::create_compiler_and_run<core[c951771928f8d9c9]::result::Result<(), rustc_errors[1ec2a23e13b589f5]::ErrorReported>, rustc_driver[55b90faad7aa1d7b]::run_compiler::{closure#1}>::{closure#1}>
  16:        0x117e9c681 - rustc_interface[112a8d2e82d7db12]::interface::create_compiler_and_run::<core[c951771928f8d9c9]::result::Result<(), rustc_errors[1ec2a23e13b589f5]::ErrorReported>, rustc_driver[55b90faad7aa1d7b]::run_compiler::{closure#1}>
  17:        0x117e81785 - <scoped_tls[77dae606f0013de7]::ScopedKey<rustc_span[be5d9d9c22212131]::SessionGlobals>>::set::<rustc_interface[112a8d2e82d7db12]::interface::run_compiler<core[c951771928f8d9c9]::result::Result<(), rustc_errors[1ec2a23e13b589f5]::ErrorReported>, rustc_driver[55b90faad7aa1d7b]::run_compiler::{closure#1}>::{closure#0}, core[c951771928f8d9c9]::result::Result<(), rustc_errors[1ec2a23e13b589f5]::ErrorReported>>
  18:        0x117e7edd7 - std[7522013a281df534]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[112a8d2e82d7db12]::util::run_in_thread_pool_with_globals<rustc_interface[112a8d2e82d7db12]::interface::run_compiler<core[c951771928f8d9c9]::result::Result<(), rustc_errors[1ec2a23e13b589f5]::ErrorReported>, rustc_driver[55b90faad7aa1d7b]::run_compiler::{closure#1}>::{closure#0}, core[c951771928f8d9c9]::result::Result<(), rustc_errors[1ec2a23e13b589f5]::ErrorReported>>::{closure#0}, core[c951771928f8d9c9]::result::Result<(), rustc_errors[1ec2a23e13b589f5]::ErrorReported>>
  19:        0x117e7b031 - <<std[7522013a281df534]::thread::Builder>::spawn_unchecked_<rustc_interface[112a8d2e82d7db12]::util::run_in_thread_pool_with_globals<rustc_interface[112a8d2e82d7db12]::interface::run_compiler<core[c951771928f8d9c9]::result::Result<(), rustc_errors[1ec2a23e13b589f5]::ErrorReported>, rustc_driver[55b90faad7aa1d7b]::run_compiler::{closure#1}>::{closure#0}, core[c951771928f8d9c9]::result::Result<(), rustc_errors[1ec2a23e13b589f5]::ErrorReported>>::{closure#0}, core[c951771928f8d9c9]::result::Result<(), rustc_errors[1ec2a23e13b589f5]::ErrorReported>>::{closure#1} as core[c951771928f8d9c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:        0x10e7516a7 - std::sys::unix::thread::Thread::new::thread_start::h6dc4137c539c4ff0
  21:     0x7ff8163424f4 - __pthread_start

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (d3ad51b48 2022-02-25) running on x86_64-apple-darwin

note: compiler flags: --crate-type bin -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
warning: `catnip` (example "packet") generated 3 warnings
error: could not compile `catnip`; 3 warnings emitted
