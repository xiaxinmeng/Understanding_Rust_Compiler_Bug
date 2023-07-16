plain
    Checking chalk-engine v0.76.0
error[E0422]: cannot find struct, variant or union type `TargetOptions` in this scope
  --> compiler/rustc_target/src/spec/i686_linux_android.rs:24:18
   |
24 |         options: TargetOptions { supported_sanitizers: SanitizerSet::ADDRESS, ..base },
   |
help: consider importing this struct
   |
1  | use crate::spec::TargetOptions;
1  | use crate::spec::TargetOptions;
   |

error[E0422]: cannot find struct, variant or union type `TargetOptions` in this scope
  --> compiler/rustc_target/src/spec/x86_64_linux_android.rs:19:18
   |
19 |         options: TargetOptions { supported_sanitizers: SanitizerSet::ADDRESS, ..base },
   |
help: consider importing this struct
   |
1  | use crate::spec::TargetOptions;
