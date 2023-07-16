plain
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `crate::panic::panic_any`
 --> library/core/src/error.rs:1:1
  |
1 | #![doc = include_str!("error.md")]
  |
  |
  = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
  = note: the link appears in this line:
          
          [`panic_any`]: crate::panic::panic_any
                         ^^^^^^^^^^^^^^^^^^^^^^^
  = note: no item named `panic_any` in module `panic`
error: unresolved link to `crate::panic::set_hook`
 --> library/core/src/error.rs:1:1
  |
  |
1 | #![doc = include_str!("error.md")]
  |
  = note: the link appears in this line:
          
          
          [`set_hook`]: crate::panic::set_hook
                        ^^^^^^^^^^^^^^^^^^^^^^
  = note: no item named `set_hook` in module `panic`
error: unresolved link to `crate::panic::take_hook`
 --> library/core/src/error.rs:1:1
  |
  |
1 | #![doc = include_str!("error.md")]
  |
  = note: the link appears in this line:
          
          
          [`take_hook`]: crate::panic::take_hook
                         ^^^^^^^^^^^^^^^^^^^^^^^
  = note: no item named `take_hook` in module `panic`
error: unresolved link to `crate::panic::catch_unwind`
 --> library/core/src/error.rs:1:1
  |
  |
1 | #![doc = include_str!("error.md")]
  |
  = note: the link appears in this line:
          
          
          [`catch_unwind`]: crate::panic::catch_unwind
  = note: no item named `catch_unwind` in module `panic`

error: unresolved link to `crate::panic::resume_unwind`
 --> library/core/src/error.rs:1:1
 --> library/core/src/error.rs:1:1
  |
1 | #![doc = include_str!("error.md")]
  |
  = note: the link appears in this line:
          
          
          [`resume_unwind`]: crate::panic::resume_unwind
  = note: no item named `resume_unwind` in module `panic`

error: unresolved link to `crate::process::Termination`
 --> library/core/src/error.rs:1:1
 --> library/core/src/error.rs:1:1
  |
1 | #![doc = include_str!("error.md")]
  |
  = note: the link appears in this line:
          
          
          [`Termination`]: crate::process::Termination
  = note: no item named `process` in module `core`

error: could not document `core`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.65.0 --index-page /checkout/src/doc/index.md -C metadata=b1f251f1f3d92979 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.65.0-nightly
  (74fff1049
  2022-08-17)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)
