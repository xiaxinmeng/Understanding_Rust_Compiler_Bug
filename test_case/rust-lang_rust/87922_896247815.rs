plain
   |
55 | mod arm_base;
   | ^^^^^^^^^^^^^
   |
   = help: to create the module `arm_base`, create file "compiler/rustc_target/src/spec/arm_base.rs" or "compiler/rustc_target/src/spec/arm_base/mod.rs"

error[E0583]: file not found for module `riscv_base`
  --> compiler/rustc_target/src/spec/mod.rs:74:1
   |
74 | mod riscv_base;
   |
   |
   = help: to create the module `riscv_base`, create file "compiler/rustc_target/src/spec/riscv_base.rs" or "compiler/rustc_target/src/spec/riscv_base/mod.rs"
error[E0583]: file not found for module `wasm32_base`
  --> compiler/rustc_target/src/spec/mod.rs:79:1
   |
   |
79 | mod wasm32_base;
   |
   |
   = help: to create the module `wasm32_base`, create file "compiler/rustc_target/src/spec/wasm32_base.rs" or "compiler/rustc_target/src/spec/wasm32_base/mod.rs"
error[E0583]: file not found for module `x86_64_linux_kernel`
   --> compiler/rustc_target/src/spec/mod.rs:447:11
    |
445 | / macro_rules! supported_targets {
445 | / macro_rules! supported_targets {
446 | |     ( $(($( $triple:literal, )+ $module:ident ),)+ ) => {
447 | |         $(mod $module;)+
448 | |
...   |
474 | |     };
475 | | }
475 | | }
    | |_- in this expansion of `supported_targets!`
477 | / supported_targets! {
477 | / supported_targets! {
478 | |     ("x86_64-unknown-linux-gnu", x86_64_unknown_linux_gnu),
479 | |     ("x86_64-unknown-linux-gnux32", x86_64_unknown_linux_gnux32),
480 | |     ("i686-unknown-linux-gnu", i686_unknown_linux_gnu),
...   |
664 | |     ("thumbv4t-none-eabi", thumbv4t_none_eabi),
    | |_- in this macro invocation
    |
    |
    = help: to create the module `x86_64_linux_kernel`, create file "compiler/rustc_target/src/spec/x86_64_linux_kernel.rs" or "compiler/rustc_target/src/spec/x86_64_linux_kernel/mod.rs"
error[E0583]: file not found for module `x86_64_rumprun_netbsd`
   --> compiler/rustc_target/src/spec/mod.rs:447:11
    |
445 | / macro_rules! supported_targets {
445 | / macro_rules! supported_targets {
446 | |     ( $(($( $triple:literal, )+ $module:ident ),)+ ) => {
447 | |         $(mod $module;)+
448 | |
...   |
474 | |     };
475 | | }
475 | | }
    | |_- in this expansion of `supported_targets!`
477 | / supported_targets! {
477 | / supported_targets! {
478 | |     ("x86_64-unknown-linux-gnu", x86_64_unknown_linux_gnu),
479 | |     ("x86_64-unknown-linux-gnux32", x86_64_unknown_linux_gnux32),
480 | |     ("i686-unknown-linux-gnu", i686_unknown_linux_gnu),
...   |
664 | |     ("thumbv4t-none-eabi", thumbv4t_none_eabi),
    | |_- in this macro invocation
    |
    |
    = help: to create the module `x86_64_rumprun_netbsd`, create file "compiler/rustc_target/src/spec/x86_64_rumprun_netbsd.rs" or "compiler/rustc_target/src/spec/x86_64_rumprun_netbsd/mod.rs"
error[E0583]: file not found for module `x86_64_unknown_hermit_kernel`
   --> compiler/rustc_target/src/spec/mod.rs:447:11
    |
445 | / macro_rules! supported_targets {
445 | / macro_rules! supported_targets {
446 | |     ( $(($( $triple:literal, )+ $module:ident ),)+ ) => {
447 | |         $(mod $module;)+
448 | |
...   |
474 | |     };
475 | | }
475 | | }
    | |_- in this expansion of `supported_targets!`
477 | / supported_targets! {
477 | / supported_targets! {
478 | |     ("x86_64-unknown-linux-gnu", x86_64_unknown_linux_gnu),
479 | |     ("x86_64-unknown-linux-gnux32", x86_64_unknown_linux_gnux32),
480 | |     ("i686-unknown-linux-gnu", i686_unknown_linux_gnu),
...   |
664 | |     ("thumbv4t-none-eabi", thumbv4t_none_eabi),
    | |_- in this macro invocation
    |
    |
    = help: to create the module `x86_64_unknown_hermit_kernel`, create file "compiler/rustc_target/src/spec/x86_64_unknown_hermit_kernel.rs" or "compiler/rustc_target/src/spec/x86_64_unknown_hermit_kernel/mod.rs"

error[E0432]: unresolved imports `crate::spec::FramePointer`, `crate::spec::SplitDebuginfo`
 --> compiler/rustc_target/src/spec/apple_base.rs:3:19
  |
3 | use crate::spec::{FramePointer, SplitDebuginfo, TargetOptions};
  |                   ^^^^^^^^^^^^  ^^^^^^^^^^^^^^ no `SplitDebuginfo` in `spec`
  |                   |
  |                   no `FramePointer` in `spec`
error[E0432]: unresolved import `crate::spec::FramePointer`
 --> compiler/rustc_target/src/spec/illumos_base.rs:1:19
  |
  |
1 | use crate::spec::{FramePointer, LinkArgs, LinkerFlavor, TargetOptions};
  |                   ^^^^^^^^^^^^ no `FramePointer` in `spec`

error[E0432]: unresolved imports `crate::spec::FramePointer`, `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/linux_kernel_base.rs:2:19
  |
2 | use crate::spec::{FramePointer, PanicStrategy, RelocModel, RelroLevel, StackProbeType};
  |                   ^^^^^^^^^^^^ no `FramePointer` in `spec`             ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::SplitDebuginfo`
error[E0432]: unresolved import `crate::spec::SplitDebuginfo`
 --> compiler/rustc_target/src/spec/msvc_base.rs:1:54
  |
1 | use crate::spec::{LinkArgs, LinkerFlavor, LldFlavor, SplitDebuginfo, TargetOptions};
  |                                                      ^^^^^^^^^^^^^^ no `SplitDebuginfo` in `spec`
error[E0432]: unresolved import `crate::spec::FramePointer`
 --> compiler/rustc_target/src/spec/openbsd_base.rs:1:19
  |
  |
1 | use crate::spec::{FramePointer, RelroLevel, TargetOptions};
  |                   ^^^^^^^^^^^^ no `FramePointer` in `spec`
error[E0432]: unresolved import `crate::spec::FramePointer`
  --> compiler/rustc_target/src/spec/thumb_base.rs:31:19
   |
   |
31 | use crate::spec::{FramePointer, LinkerFlavor, LldFlavor, PanicStrategy, RelocModel};
   |                   ^^^^^^^^^^^^ no `FramePointer` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
error[E0432]: unresolved import `crate::spec::StackProbeType`
  --> compiler/rustc_target/src/spec/uefi_msvc_base.rs:12:59
   |
12 | use crate::spec::{LinkerFlavor, LldFlavor, PanicStrategy, StackProbeType, TargetOptions};
   |                                                           ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`

error[E0432]: unresolved imports `crate::spec::SanitizerSet`, `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_unknown_linux_gnu.rs:1:33
  |
1 | use crate::spec::{LinkerFlavor, SanitizerSet, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^  ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
  |                                 |
  |                                 no `SanitizerSet` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_unknown_linux_gnux32.rs:1:33
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/i686_unknown_linux_gnu.rs:1:33
  |
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::SanitizerSet`
 --> compiler/rustc_target/src/spec/aarch64_unknown_linux_gnu.rs:1:19
  |
  |
1 | use crate::spec::{SanitizerSet, Target, TargetOptions};
  |                   ^^^^^^^^^^^^ no `SanitizerSet` in `spec`

error[E0432]: unresolved imports `crate::spec::SanitizerSet`, `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_unknown_linux_musl.rs:1:33
  |
1 | use crate::spec::{LinkerFlavor, SanitizerSet, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^  ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
  |                                 |
  |                                 no `SanitizerSet` in `spec`

error[E0432]: unresolved imports `crate::spec::FramePointer`, `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/i686_unknown_linux_musl.rs:1:19
  |
1 | use crate::spec::{FramePointer, LinkerFlavor, StackProbeType, Target};
  |                   ^^^^^^^^^^^^                ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
  |                   |
  |                   no `FramePointer` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/i686_linux_android.rs:1:19
  |
  |
1 | use crate::spec::{StackProbeType, Target};
  |                   ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_linux_android.rs:1:33
  |
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::SanitizerSet`
 --> compiler/rustc_target/src/spec/aarch64_linux_android.rs:1:19
  |
  |
1 | use crate::spec::{SanitizerSet, Target, TargetOptions};
  |                   ^^^^^^^^^^^^ no `SanitizerSet` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/i686_unknown_freebsd.rs:1:33
  |
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`

error[E0432]: unresolved imports `crate::spec::SanitizerSet`, `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_unknown_freebsd.rs:1:33
  |
1 | use crate::spec::{LinkerFlavor, SanitizerSet, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^  ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
  |                                 |
  |                                 no `SanitizerSet` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_unknown_dragonfly.rs:1:33
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/i686_unknown_openbsd.rs:1:33
  |
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_unknown_openbsd.rs:1:33
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/i686_unknown_netbsd.rs:1:33
  |
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target, TargetOptions};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`

error[E0432]: unresolved imports `crate::spec::SanitizerSet`, `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_unknown_netbsd.rs:1:33
  |
1 | use crate::spec::{LinkerFlavor, SanitizerSet, StackProbeType, Target, TargetOptions};
  |                                 ^^^^^^^^^^^^  ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
  |                                 |
  |                                 no `SanitizerSet` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/i686_unknown_haiku.rs:1:33
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_unknown_haiku.rs:1:33
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`

error[E0432]: unresolved imports `crate::spec::FramePointer`, `crate::spec::SanitizerSet`
 --> compiler/rustc_target/src/spec/aarch64_apple_darwin.rs:1:19
  |
1 | use crate::spec::{FramePointer, LinkerFlavor, SanitizerSet, Target, TargetOptions};
  |                   ^^^^^^^^^^^^                ^^^^^^^^^^^^ no `SanitizerSet` in `spec`
  |                   |
  |                   no `FramePointer` in `spec`

error[E0432]: unresolved imports `crate::spec::FramePointer`, `crate::spec::SanitizerSet`, `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_apple_darwin.rs:2:19
  |
2 | use crate::spec::{FramePointer, LinkerFlavor, SanitizerSet, StackProbeType, Target};
  |                   ^^^^^^^^^^^^                ^^^^^^^^^^^^  ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
  |                   |                           |
  |                   |                           no `SanitizerSet` in `spec`
  |                   no `FramePointer` in `spec`

error[E0432]: unresolved imports `crate::spec::FramePointer`, `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/i686_apple_darwin.rs:1:19
  |
1 | use crate::spec::{FramePointer, LinkerFlavor, StackProbeType, Target, TargetOptions};
  |                   ^^^^^^^^^^^^                ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
  |                   |
  |                   no `FramePointer` in `spec`
error[E0432]: unresolved import `crate::spec::SanitizerSet`
 --> compiler/rustc_target/src/spec/aarch64_fuchsia.rs:1:19
  |
  |
1 | use crate::spec::{SanitizerSet, Target, TargetOptions};
  |                   ^^^^^^^^^^^^ no `SanitizerSet` in `spec`

error[E0432]: unresolved imports `crate::spec::SanitizerSet`, `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_fuchsia.rs:1:19
  |
1 | use crate::spec::{SanitizerSet, StackProbeType, Target};
  |                   ^^^^^^^^^^^^  ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
  |                   |
  |                   no `SanitizerSet` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_unknown_redox.rs:1:33
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/i386_apple_ios.rs:2:19
  |
  |
2 | use crate::spec::{StackProbeType, Target, TargetOptions};
  |                   ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_apple_ios.rs:2:19
  |
2 | use crate::spec::{StackProbeType, Target, TargetOptions};
  |                   ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::FramePointer`
 --> compiler/rustc_target/src/spec/aarch64_apple_ios.rs:2:19
  |
  |
2 | use crate::spec::{FramePointer, Target, TargetOptions};
  |                   ^^^^^^^^^^^^ no `FramePointer` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_apple_ios_macabi.rs:2:19
  |
2 | use crate::spec::{StackProbeType, Target, TargetOptions};
  |                   ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::FramePointer`
 --> compiler/rustc_target/src/spec/aarch64_apple_ios_macabi.rs:2:19
  |
  |
2 | use crate::spec::{FramePointer, Target, TargetOptions};
  |                   ^^^^^^^^^^^^ no `FramePointer` in `spec`
error[E0432]: unresolved import `crate::spec::FramePointer`
 --> compiler/rustc_target/src/spec/aarch64_apple_tvos.rs:2:19
  |
  |
2 | use crate::spec::{FramePointer, Target, TargetOptions};
  |                   ^^^^^^^^^^^^ no `FramePointer` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_apple_tvos.rs:2:19
  |
2 | use crate::spec::{StackProbeType, Target, TargetOptions};
  |                   ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_sun_solaris.rs:1:33
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::FramePointer`
 --> compiler/rustc_target/src/spec/i686_pc_windows_gnu.rs:1:19
  |
  |
1 | use crate::spec::{FramePointer, LinkerFlavor, LldFlavor, Target};
  |                   ^^^^^^^^^^^^ no `FramePointer` in `spec`
error[E0432]: unresolved import `crate::spec::FramePointer`
 --> compiler/rustc_target/src/spec/i686_uwp_windows_gnu.rs:1:19
  |
  |
1 | use crate::spec::{FramePointer, LinkerFlavor, LldFlavor, Target};
  |                   ^^^^^^^^^^^^ no `FramePointer` in `spec`

error[E0432]: unresolved import `super::wasm_base`
 --> compiler/rustc_target/src/spec/wasm32_unknown_emscripten.rs:1:5
1 | use super::wasm_base;
  |     ^^^^^^^---------
  |     |      |
  |     |      help: a similar name exists in the module: `arm_base`
  |     |      help: a similar name exists in the module: `arm_base`
  |     no `wasm_base` in `spec`

error[E0432]: unresolved import `super::wasm_base`
  --> compiler/rustc_target/src/spec/wasm32_unknown_unknown.rs:13:5
13 | use super::wasm_base;
   |     ^^^^^^^---------
   |     |      |
   |     |      help: a similar name exists in the module: `arm_base`
   |     |      help: a similar name exists in the module: `arm_base`
   |     no `wasm_base` in `spec`

error[E0432]: unresolved import `super::wasm_base`
  --> compiler/rustc_target/src/spec/wasm32_wasi.rs:75:5
75 | use super::wasm_base;
   |     ^^^^^^^---------
   |     |      |
   |     |      help: a similar name exists in the module: `arm_base`
   |     |      help: a similar name exists in the module: `arm_base`
   |     no `wasm_base` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_unknown_hermit.rs:1:19
  |
1 | use crate::spec::{StackProbeType, Target};
  |                   ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/i686_wrs_vxworks.rs:1:33
  |
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`
error[E0432]: unresolved import `crate::spec::StackProbeType`
 --> compiler/rustc_target/src/spec/x86_64_wrs_vxworks.rs:1:33
  |
  |
1 | use crate::spec::{LinkerFlavor, StackProbeType, Target};
  |                                 ^^^^^^^^^^^^^^ no `StackProbeType` in `spec`

error[E0425]: cannot find function `target` in module `x86_64_linux_kernel`
   --> compiler/rustc_target/src/spec/mod.rs:454:45
445 | / macro_rules! supported_targets {
445 | / macro_rules! supported_targets {
446 | |     ( $(($( $triple:literal, )+ $module:ident ),)+ ) => {
447 | |         $(mod $module;)+
...   |
...   |
454 | |                 $( $($triple)|+ => $module::target(), )+
    | |                                             ^^^^^^ not found in `x86_64_linux_kernel`
474 | |     };
475 | | }
475 | | }
    | |_- in this expansion of `supported_targets!`
477 | / supported_targets! {
477 | / supported_targets! {
478 | |     ("x86_64-unknown-linux-gnu", x86_64_unknown_linux_gnu),
479 | |     ("x86_64-unknown-linux-gnux32", x86_64_unknown_linux_gnux32),
480 | |     ("i686-unknown-linux-gnu", i686_unknown_linux_gnu),
...   |
664 | |     ("thumbv4t-none-eabi", thumbv4t_none_eabi),
    | |_- in this macro invocation
    |
help: consider importing one of these items
    |
---
37  | use crate::spec::aarch64_apple_tvos::target;
    |
      and 148 other candidates

error[E0425]: cannot find function `target` in module `x86_64_rumprun_netbsd`
   --> compiler/rustc_target/src/spec/mod.rs:454:45
445 | / macro_rules! supported_targets {
445 | / macro_rules! supported_targets {
446 | |     ( $(($( $triple:literal, )+ $module:ident ),)+ ) => {
447 | |         $(mod $module;)+
...   |
...   |
454 | |                 $( $($triple)|+ => $module::target(), )+
    | |                                             ^^^^^^ not found in `x86_64_rumprun_netbsd`
474 | |     };
475 | | }
475 | | }
    | |_- in this expansion of `supported_targets!`
477 | / supported_targets! {
477 | / supported_targets! {
478 | |     ("x86_64-unknown-linux-gnu", x86_64_unknown_linux_gnu),
479 | |     ("x86_64-unknown-linux-gnux32", x86_64_unknown_linux_gnux32),
480 | |     ("i686-unknown-linux-gnu", i686_unknown_linux_gnu),
...   |
664 | |     ("thumbv4t-none-eabi", thumbv4t_none_eabi),
    | |_- in this macro invocation
    |
help: consider importing one of these items
    |
---
37  | use crate::spec::aarch64_apple_tvos::target;
    |
      and 148 other candidates

error[E0425]: cannot find function `target` in module `x86_64_unknown_hermit_kernel`
   --> compiler/rustc_target/src/spec/mod.rs:454:45
445 | / macro_rules! supported_targets {
445 | / macro_rules! supported_targets {
446 | |     ( $(($( $triple:literal, )+ $module:ident ),)+ ) => {
447 | |         $(mod $module;)+
...   |
...   |
454 | |                 $( $($triple)|+ => $module::target(), )+
    | |                                             ^^^^^^ not found in `x86_64_unknown_hermit_kernel`
474 | |     };
475 | | }
475 | | }
    | |_- in this expansion of `supported_targets!`
477 | / supported_targets! {
477 | / supported_targets! {
478 | |     ("x86_64-unknown-linux-gnu", x86_64_unknown_linux_gnu),
479 | |     ("x86_64-unknown-linux-gnux32", x86_64_unknown_linux_gnux32),
480 | |     ("i686-unknown-linux-gnu", i686_unknown_linux_gnu),
...   |
664 | |     ("thumbv4t-none-eabi", thumbv4t_none_eabi),
    | |_- in this macro invocation
    |
help: consider importing one of these items
    |
---

error[E0423]: expected value, found struct variant `Abi::Stdcall`
    --> compiler/rustc_target/src/spec/mod.rs:1134:21
     |
1134 |                     Abi::Stdcall
     |                     ^^^^^^^^^^^^ help: use struct literal syntax instead: `Abi::Stdcall { unwind: val }`
    ::: compiler/rustc_target/src/spec/abi.rs:17:5
     |
17   |     Stdcall { unwind: bool },
     |     ------------------------ `Abi::Stdcall` defined here
---

error[E0532]: expected unit struct, unit variant or constant, found struct variant `Abi::Stdcall`
    --> compiler/rustc_target/src/spec/mod.rs:1142:13
     |
1142 |             Abi::Stdcall | Abi::Fastcall | Abi::Vectorcall | Abi::Thiscall => {
     |             ^^^^^^^^^^^^ help: use struct pattern syntax instead: `Abi::Stdcall { unwind }`
    ::: compiler/rustc_target/src/spec/abi.rs:17:5
     |
17   |     Stdcall { unwind: bool },
     |     ------------------------ `Abi::Stdcall` defined here
     |     ------------------------ `Abi::Stdcall` defined here

error[E0532]: expected unit struct, unit variant or constant, found struct variant `Abi::Thiscall`
    --> compiler/rustc_target/src/spec/mod.rs:1142:62
     |
1142 |             Abi::Stdcall | Abi::Fastcall | Abi::Vectorcall | Abi::Thiscall => {
     |                                                              ^^^^^^^^^^^^^ help: use struct pattern syntax instead: `Abi::Thiscall { unwind }`
    ::: compiler/rustc_target/src/spec/abi.rs:20:5
     |
20   |     Thiscall { unwind: bool },
     |     ------------------------- `Abi::Thiscall` defined here
     |     ------------------------- `Abi::Thiscall` defined here

error[E0423]: expected value, found struct variant `Abi::C`
    --> compiler/rustc_target/src/spec/mod.rs:1143:65
     |
1143 |                 if self.is_like_windows && self.arch != "x86" { Abi::C } else { abi }
     | 
    ::: compiler/rustc_target/src/spec/abi.rs:15:5
     |
15   |     C { unwind: bool },
15   |     C { unwind: bool },
     |     ------------------ `Abi::C` defined here
     |
help: use struct literal syntax instead
     |
1143 |                 if self.is_like_windows && self.arch != "x86" { Abi::C { unwind: val } } else { abi }
help: consider importing one of these items instead
     |
37   | use crate::abi::call::Conv::C;
     |
---

error[E0308]: mismatched types
   --> compiler/rustc_target/src/abi/mod.rs:162:25
    |
162 |         if dl.endian != target.endian {
    |                         ^^^^^^^^^^^^^ expected enum `Endian`, found struct `std::string::String`
error[E0308]: mismatched types
   --> compiler/rustc_target/src/abi/mod.rs:180:49
    |
    |
180 |         dl.c_enum_min_size = Integer::from_bits(&target.c_enum_min_bits);
    |                                                 |
    |                                                 |
    |                                                 expected `u64`, found `&u64`
    |                                                 help: consider removing the borrow: `target.c_enum_min_bits`
error[E0308]: mismatched types
   --> compiler/rustc_target/src/abi/mod.rs:180:30
    |
    |
180 |         dl.c_enum_min_size = Integer::from_bits(&target.c_enum_min_bits);
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Integer`, found enum `Result`
    = note: expected enum `Integer`
               found enum `Result<Integer, std::string::String>`

    Checking tracing-serde v0.1.2
    Checking tracing-serde v0.1.2
    Checking gsgdt v0.1.2
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
error[E0599]: no method named `generic` found for enum `spec::abi::Abi` in the current scope
    --> compiler/rustc_target/src/spec/mod.rs:1169:13
     |
1169 |         abi.generic() || !self.unsupported_abis.contains(&abi)
     |             ^^^^^^^ method not found in `spec::abi::Abi`
    ::: compiler/rustc_target/src/spec/abi.rs:10:1
     |
10   | pub enum Abi {
10   | pub enum Abi {
     | ------------ method `generic` not found for this

error[E0599]: no method named `generic` found for enum `spec::abi::Abi` in the current scope
    --> compiler/rustc_target/src/spec/mod.rs:1545:36
     |
1545 | ...                   if abi.generic() {
     |                              ^^^^^^^ method not found in `spec::abi::Abi`
    ::: compiler/rustc_target/src/spec/abi.rs:10:1
     |
10   | pub enum Abi {
10   | pub enum Abi {
     | ------------ method `generic` not found for this
    Checking tracing-subscriber v0.2.16
    Checking tracing-subscriber v0.2.16
error[E0599]: no variant or associated item named `WasiReactorExe` found for enum `LinkOutputKind` in the current scope
   --> compiler/rustc_target/src/spec/crt_objects.rs:118:26
    |
118 |         (LinkOutputKind::WasiReactorExe, &["crt1-reactor.o"]),
    |                          ^^^^^^^^^^^^^^ variant or associated item not found in `LinkOutputKind`
   ::: compiler/rustc_target/src/spec/mod.rs:397:1
    |
    |
397 | pub enum LinkOutputKind {
    | ----------------------- variant or associated item `WasiReactorExe` not found here
error[E0609]: no field `default_uwtable` on type `TargetOptions`
  --> compiler/rustc_target/src/spec/android_base.rs:18:10
   |
   |
18 |     base.default_uwtable = true;
   |
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
  --> compiler/rustc_target/src/spec/apple_base.rs:27:9
   |
27 |         families: vec!["unix".to_string()],
   |         ^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `frame_pointer`
  --> compiler/rustc_target/src/spec/apple_base.rs:30:9
   |
   |
30 |         frame_pointer: FramePointer::Always,
   |         ^^^^^^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `split_debuginfo`
  --> compiler/rustc_target/src/spec/apple_base.rs:41:9
   |
41 |         split_debuginfo: SplitDebuginfo::Packed,
41 |         split_debuginfo: SplitDebuginfo::Packed,
   |         ^^^^^^^^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/apple_sdk_base.rs:51:9
   |
51 |         abi: target_abi(arch),
   |         ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
 --> compiler/rustc_target/src/spec/dragonfly_base.rs:8:9
  |
8 |         families: vec!["unix".to_string()],
  |         ^^^^^^^^ `TargetOptions` does not have this field
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
 --> compiler/rustc_target/src/spec/freebsd_base.rs:8:9
  |
8 |         families: vec!["unix".to_string()],
  |         ^^^^^^^^ `TargetOptions` does not have this field
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
  --> compiler/rustc_target/src/spec/fuchsia_base.rs:28:9
   |
28 |         families: vec!["unix".to_string()],
   |         ^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
 --> compiler/rustc_target/src/spec/haiku_base.rs:8:9
  |
8 |         families: vec!["unix".to_string()],
  |         ^^^^^^^^ `TargetOptions` does not have this field
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
  --> compiler/rustc_target/src/spec/illumos_base.rs:34:9
   |
34 |         families: vec!["unix".to_string()],
   |         ^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `frame_pointer`
  --> compiler/rustc_target/src/spec/illumos_base.rs:38:9
   |
   |
38 |         frame_pointer: FramePointer::Always,
   |         ^^^^^^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
  --> compiler/rustc_target/src/spec/l4re_base.rs:24:9
   |
24 |         families: vec!["unix".to_string()],
   |         ^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
 --> compiler/rustc_target/src/spec/linux_base.rs:8:9
  |
8 |         families: vec!["unix".to_string()],
  |         ^^^^^^^^ `TargetOptions` does not have this field
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `frame_pointer`
  --> compiler/rustc_target/src/spec/linux_kernel_base.rs:11:9
   |
   |
11 |         frame_pointer: FramePointer::Always,
   |         ^^^^^^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `split_debuginfo`
error[E0560]: struct `TargetOptions` has no field named `split_debuginfo`
  --> compiler/rustc_target/src/spec/msvc_base.rs:26:9
26 |         split_debuginfo: SplitDebuginfo::Packed,
   |         ^^^^^^^^^^^^^^^ `TargetOptions` does not have this field
   |
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
 --> compiler/rustc_target/src/spec/netbsd_base.rs:8:9
  |
8 |         families: vec!["unix".to_string()],
  |         ^^^^^^^^ `TargetOptions` does not have this field
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
 --> compiler/rustc_target/src/spec/openbsd_base.rs:8:9
  |
8 |         families: vec!["unix".to_string()],
  |         ^^^^^^^^ `TargetOptions` does not have this field
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `frame_pointer`
  --> compiler/rustc_target/src/spec/openbsd_base.rs:12:9
   |
   |
12 |         frame_pointer: FramePointer::Always, // FIXME 43575: should be MayOmit...
   |         ^^^^^^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
 --> compiler/rustc_target/src/spec/redox_base.rs:9:9
  |
9 |         families: vec!["unix".to_string()],
  |         ^^^^^^^^ `TargetOptions` does not have this field
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
 --> compiler/rustc_target/src/spec/solaris_base.rs:9:9
  |
9 |         families: vec!["unix".to_string()],
  |         ^^^^^^^^ `TargetOptions` does not have this field
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `frame_pointer`
  --> compiler/rustc_target/src/spec/thumb_base.rs:55:9
   |
   |
55 |         frame_pointer: FramePointer::Always,
   |         ^^^^^^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
  --> compiler/rustc_target/src/spec/vxworks_base.rs:12:9
   |
12 |         families: vec!["unix".to_string()],
   |         ^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
  --> compiler/rustc_target/src/spec/windows_gnu_base.rs:74:9
   |
74 |         families: vec!["windows".to_string()],
   |         ^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0560]: struct `TargetOptions` has no field named `families`
  --> compiler/rustc_target/src/spec/windows_msvc_base.rs:16:9
   |
16 |         families: vec!["windows".to_string()],
   |         ^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/windows_uwp_gnu_base.rs:28:9
   |
28 |         abi: "uwp".to_string(),
   |         ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `abi` on type `TargetOptions`
 --> compiler/rustc_target/src/spec/windows_uwp_msvc_base.rs:6:10
  |
6 |     opts.abi = "uwp".to_string();
  |
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `supported_sanitizers` on type `TargetOptions`
  --> compiler/rustc_target/src/spec/x86_64_unknown_linux_gnu.rs:10:10
10 |     base.supported_sanitizers =
   |          ^^^^^^^^^^^^^^^^^^^^ unknown field
   |
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others

error[E0609]: no field `abi` on type `TargetOptions`
 --> compiler/rustc_target/src/spec/x86_64_unknown_linux_gnux32.rs:6:10
  |
6 |     base.abi = "x32".to_string();
  |
  |
  = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/mips_unknown_linux_gnu.rs:11:21
   |
11 |             endian: Endian::Big,
11 |             endian: Endian::Big,
   |                     ^^^^^^^^^^^ expected struct `std::string::String`, found enum `Endian`
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/mips64_unknown_linux_gnuabi64.rs:11:13
   |
   |
11 |             abi: "abi64".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/mips64_unknown_linux_gnuabi64.rs:12:21
   |
12 |             endian: Endian::Big,
12 |             endian: Endian::Big,
   |                     ^^^^^^^^^^^ expected struct `std::string::String`, found enum `Endian`
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/mips64el_unknown_linux_gnuabi64.rs:10:13
   |
   |
10 |             abi: "abi64".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/mipsisa32r6_unknown_linux_gnu.rs:11:21
   |
11 |             endian: Endian::Big,
11 |             endian: Endian::Big,
   |                     ^^^^^^^^^^^ expected struct `std::string::String`, found enum `Endian`
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/mipsisa64r6_unknown_linux_gnuabi64.rs:11:13
   |
   |
11 |             abi: "abi64".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/mipsisa64r6_unknown_linux_gnuabi64.rs:12:21
   |
12 |             endian: Endian::Big,
12 |             endian: Endian::Big,
   |                     ^^^^^^^^^^^ expected struct `std::string::String`, found enum `Endian`
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/mipsisa64r6el_unknown_linux_gnuabi64.rs:10:13
   |
   |
10 |             abi: "abi64".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/powerpc_unknown_linux_gnu.rs:14:42
   |
   |
14 |         options: TargetOptions { endian: Endian::Big, mcount: "_mcount".to_string(), ..base },
   |                                          ^^^^^^^^^^^ expected struct `std::string::String`, found enum `Endian`
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/powerpc_unknown_linux_gnuspe.rs:15:13
   |
   |
15 |             abi: "spe".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/powerpc_unknown_linux_gnuspe.rs:16:21
   |
16 |             endian: Endian::Big,
16 |             endian: Endian::Big,
   |                     ^^^^^^^^^^^ expected struct `std::string::String`, found enum `Endian`
error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/powerpc_unknown_linux_musl.rs:14:42
   |
   |
14 |         options: TargetOptions { endian: Endian::Big, mcount: "_mcount".to_string(), ..base },
   |                                          ^^^^^^^^^^^ expected struct `std::string::String`, found enum `Endian`
error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/powerpc64_unknown_linux_gnu.rs:19:42
   |
   |
19 |         options: TargetOptions { endian: Endian::Big, mcount: "_mcount".to_string(), ..base },
   |                                          ^^^^^^^^^^^ expected struct `std::string::String`, found enum `Endian`
error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/powerpc64_unknown_linux_musl.rs:15:42
   |
   |
15 |         options: TargetOptions { endian: Endian::Big, mcount: "_mcount".to_string(), ..base },
   |                                          ^^^^^^^^^^^ expected struct `std::string::String`, found enum `Endian`
error[E0308]: mismatched types
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/s390x_unknown_linux_gnu.rs:6:19
  |
6 |     base.endian = Endian::Big;
  |                   ^^^^^^^^^^^ expected struct `std::string::String`, found enum `Endian`
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/sparc_unknown_linux_gnu.rs:6:19
  |
  |
6 |     base.endian = Endian::Big;
  |                   ^^^^^^^^^^^ expected struct `std::string::String`, found enum `Endian`
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/sparc64_unknown_linux_gnu.rs:6:19
  |
  |
6 |     base.endian = Endian::Big;
  |                   ^^^^^^^^^^^ expected struct `std::string::String`, found enum `Endian`
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/arm_unknown_linux_gnueabi.rs:10:13
   |
   |
10 |             abi: "eabi".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/arm_unknown_linux_gnueabihf.rs:10:13
   |
   |
10 |             abi: "eabihf".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/arm_unknown_linux_musleabi.rs:13:13
   |
   |
13 |             abi: "eabi".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/arm_unknown_linux_musleabihf.rs:13:13
   |
   |
13 |             abi: "eabihf".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/armv4t_unknown_linux_gnueabi.rs:10:13
   |
   |
10 |             abi: "eabi".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/armv5te_unknown_linux_gnueabi.rs:10:13
   |
   |
10 |             abi: "eabi".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/armv5te_unknown_linux_musleabi.rs:14:13
   |
   |
14 |             abi: "eabi".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/armv5te_unknown_linux_uclibceabi.rs:10:13
   |
   |
10 |             abi: "eabi".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/armv7_unknown_linux_gnueabi.rs:13:13
   |
   |
13 |             abi: "eabi".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/armv7_unknown_linux_gnueabihf.rs:13:13
   |
   |
13 |             abi: "eabihf".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/thumbv7neon_unknown_linux_gnueabihf.rs:16:13
   |
   |
16 |             abi: "eabihf".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/thumbv7neon_unknown_linux_musleabihf.rs:22:13
   |
   |
22 |             abi: "eabihf".to_string(),
   |             ^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 88 others
error[E0560]: struct `TargetOptions` has no field named `abi`
  --> compiler/rustc_target/src/spec/armv7_unknown_linux_musleabi.rs:19:13
   |
   |
19 |             abi: "eabi".to_string(),
   |             ^^^ `TargetOptions` does not have this field
