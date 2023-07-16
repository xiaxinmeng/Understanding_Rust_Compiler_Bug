plain
travis_time:end:00e77a37:start=1544474800582637503,finish=1544474802829172747,duration=2246535244
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:00:00] rm 'src/tools/lldb'
[00:00:00] Attempting with retry: sh -c rm -f download-src-tools-lldb.tar.gz &&         curl -sSL -o download-src-tools-lldb.tar.gz https://github.com/rust-lang-nursery/lldb/archive/8ad0817ce45b0eef9d374691b23f2bd69c164254.tar.gz
[00:00:00] rm 'src/tools/clang'
[00:00:00] Attempting with retry: sh -c rm -f download-src-tools-clang.tar.gz &&         curl -sSL -o download-src-tools-clang.tar.gz https://github.com/rust-lang-nursery/clang/archive/032312dd0140a7074c9b89d305fe44eb0e44e407.tar.gz
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/libbacktrace src/doc/rustc-guide src/doc/edition-guide src/rust-sgx src/doc/embedded-book &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/libbacktrace src/doc/rustc-guide src/doc/edition-guide src/rust-sgx src/doc/embedded-book
[00:00:00] Cleared directory 'src/doc/edition-guide'
[00:00:00] Cleared directory 'src/doc/embedded-book'
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
---
[00:00:00] Cleared directory 'src/tools/rust-installer'
[00:00:00] Cleared directory 'src/tools/rustfmt'
[00:00:00] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) registered for path 'src/dlmalloc'
[00:00:00] Submodule 'src/doc/edition-guide' (https://github.com/rust-lang-nursery/edition-guide) registered for path 'src/doc/edition-guide'
[00:00:00] Submodule 'src/doc/embedded-book' (https://github.com/rust-embedded/book.git) registered for path 'src/doc/embedded-book'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/doc/rustc-guide' (https://github.com/rust-lang/rustc-guide.git) registered for path 'src/doc/rustc-guide'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace.git) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:07] 
[00:54:07] running 120 tests
[00:54:10] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:54:10] ..ii.i.....iiii.....
[00:54:10] 
[00:54:10]  finished in 3.231
[00:54:10] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:24] 
[00:54:24] running 118 tests
[00:54:47] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:54:51] ......iii.i.....ii
[00:54:51] 
[00:54:51]  finished in 26.904
[00:54:51] travis_fold:end:test_debuginfo

---
[01:20:02] travis_fold:end:stage0-linkchecker

[01:20:02] travis_time:end:stage0-linkchecker:start=1544479613327137280,finish=1544479615444019918,duration=2116882638

[01:20:02] cargo/appendix/glossary.html:150: broken link - cargo/appendix/reference/manifest.html
[01:20:02] cargo/appendix/glossary.html:154: broken link - cargo/appendix/reference/manifest.html
[01:20:02] cargo/appendix/glossary.html:167: broken link - cargo/appendix/guide/cargo-toml-vs-cargo-lock.html
[01:20:02] cargo/appendix/glossary.html:169: broken link - cargo/appendix/reference/manifest.html
[01:20:02] cargo/appendix/glossary.html:171: broken link - cargo/appendix/reference/manifest.html
[01:20:02] cargo/appendix/glossary.html:182: broken link - cargo/appendix/reference/pkgid-spec.html
[01:20:02] cargo/appendix/glossary.html:198: broken link - cargo/appendix/reference/source-replacement.html
[01:20:02] cargo/appendix/glossary.html:200: broken link - cargo/appendix/reference/source-replacement.html
[01:20:02] cargo/appendix/glossary.html:202: broken link - cargo/appendix/reference/specifying-dependencies.html
[01:20:02] cargo/appendix/glossary.html:202: broken link - cargo/appendix/reference/specifying-dependencies.html
[01:20:02] cargo/appendix/glossary.html:203: broken link - cargo/appendix/reference/specifying-dependencies.html
[01:20:02] cargo/appendix/glossary.html:204: broken link - cargo/appendix/reference/source-replacement.html
[01:20:02] cargo/appendix/glossary.html:206: broken link - cargo/appendix/reference/source-replacement.html
[01:20:02] cargo/appendix/glossary.html:214: broken link - cargo/appendix/reference/manifest.html
[01:20:02] cargo/appendix/glossary.html:215: broken link - cargo/appendix/reference/manifest.html
[01:20:02] cargo/appendix/glossary.html:224: broken link - cargo/appendix/reference/config.html
[01:20:02] cargo/appendix/glossary.html:229: broken link - cargo/appendix/reference/environment-variables.html
[01:20:02] cargo/appendix/glossary.html:230: broken link - cargo/appendix/reference/config.html
[01:20:02] cargo/appendix/glossary.html:240: broken link - cargo/appendix/reference/manifest.html
[01:20:02] cargo/appendix/glossary.html:243: broken link - cargo/appendix/reference/manifest.html
[01:20:02] cargo/appendix/glossary.html:248: broken link - cargo/appendix/reference/manifest.html
[01:20:02] cargo/appendix/glossary.html:251: broken link - cargo/appendix/reference/manifest.html
[01:20:02] cargo/guide/project-layout.html:169: broken link - cargo/guide/reference/manifest.html
[01:20:02] cargo/guide/cargo-toml-vs-cargo-lock.html:151: broken link - cargo/guide/faq.html
[01:20:02] cargo/guide/cargo-toml-vs-cargo-lock.html:218: broken link - cargo/guide/reference/pkgid-spec.html
[01:20:02] cargo/guide/index.html:143: broken link - cargo/guide/guide/why-cargo-exists.html
[01:20:02] cargo/guide/index.html:144: broken link - cargo/guide/guide/creating-a-new-project.html
[01:20:02] cargo/guide/index.html:145: broken link - cargo/guide/guide/working-on-an-existing-project.html
[01:20:02] cargo/guide/index.html:146: broken link - cargo/guide/guide/dependencies.html
[01:20:02] cargo/guide/index.html:147: broken link - cargo/guide/guide/project-layout.html
[01:20:02] cargo/guide/index.html:148: broken link - cargo/guide/guide/cargo-toml-vs-cargo-lock.html
[01:20:02] cargo/guide/index.html:149: broken link - cargo/guide/guide/tests.html
[01:20:02] cargo/guide/index.html:150: broken link - cargo/guide/guide/continuous-integration.html
[01:20:02] cargo/guide/index.html:151: broken link - cargo/guide/guide/build-cache.html
[01:20:02] cargo/guide/dependencies.html:151: broken link - cargo/guide/reference/specifying-dependencies.html
[01:20:02] cargo/getting-started/index.html:142: broken link - cargo/getting-started/getting-started/installation.html
[01:20:02] cargo/getting-started/index.html:143: broken link - cargo/getting-started/getting-started/first-steps.html
[01:20:02] cargo/getting-started/first-steps.html:183: broken link - cargo/getting-started/guide/index.html
[01:20:02] cargo/reference/config.html:142: broken link - cargo/reference/reference/manifest.html
[01:20:02] cargo/reference/config.html:277: broken link - cargo/reference/reference/source-replacement.html
[01:20:02] cargo/reference/config.html:280: broken link - cargo/reference/reference/environment-variables.html
[01:20:02] cargo/reference/unstable.html:296: broken link - cargo/reference/reference/config.html
[01:20:02] cargo/reference/manifest.html:178: broken link - cargo/reference/reference/build-scripts.html
[01:20:02] cargo/reference/manifest.html:180: broken link - cargo/reference/reference/build-scripts.html
[01:20:02] cargo/reference/manifest.html:187: broken link - cargo/reference/reference/build-scripts.html
[01:20:02] cargo/reference/manifest.html:378: broken link - cargo/reference/reference/specifying-dependencies.html
[01:20:02] cargo/reference/manifest.html:785: broken link - cargo/reference/reference/specifying-dependencies.html
[01:20:02] cargo/reference/manifest.html:814: broken link - cargo/reference/reference/specifying-dependencies.html
[01:20:02] cargo/reference/manifest.html:818: broken link - cargo/reference/reference/specifying-dependencies.html
[01:20:02] cargo/reference/manifest.html:824: broken link - cargo/reference/reference/pkgid-spec.html
[01:20:02] cargo/reference/manifest.html:831: broken link - cargo/reference/reference/specifying-dependencies.html
[01:20:02] cargo/reference/publishing.html:208: broken link - cargo/reference/reference/manifest.html
[01:20:02] cargo/reference/publishing.html:213: broken link - cargo/reference/reference/manifest.html
[01:20:02] cargo/reference/build-scripts.html:169: broken link - cargo/reference/reference/environment-variables.html
[01:20:02] cargo/reference/build-scripts.html:309: broken link - cargo/reference/reference/config.html
[01:20:02] cargo/reference/index.html:142: broken link - cargo/reference/reference/specifying-dependencies.html
[01:20:02] cargo/reference/index.html:143: broken link - cargo/reference/reference/manifest.html
[01:20:02] cargo/reference/index.html:144: broken link - cargo/reference/reference/config.html
[01:20:02] cargo/reference/index.html:145: broken link - cargo/reference/reference/environment-variables.html
[01:20:02] cargo/reference/index.html:146: broken link - cargo/reference/reference/build-scripts.html
[01:20:02] cargo/reference/index.html:147: broken link - cargo/reference/reference/publishing.html
[01:20:02] cargo/reference/index.html:148: broken link - cargo/reference/reference/pkgid-spec.html
[01:20:02] cargo/reference/index.html:149: broken link - cargo/reference/reference/source-replacement.html
[01:20:02] cargo/reference/index.html:150: broken link - cargo/reference/reference/external-tools.html
[01:20:02] cargo/reference/index.html:151: broken link - cargo/reference/reference/unstable.html
[01:20:02] cargo/reference/source-replacement.html:141: broken link - cargo/reference/reference/specifying-dependencies.html
[01:20:02] cargo/reference/source-replacement.html:145: broken link - cargo/reference/reference/config.html
[01:20:02] cargo/reference/source-replacement.html:192: broken link - cargo/reference/reference/manifest.html
[01:20:02] cargo/reference/source-replacement.html:195: broken link - cargo/reference/reference/config.html
[01:20:02] cargo/reference/specifying-dependencies.html:148: broken link - cargo/reference/guide/index.html
[01:20:02] cargo/reference/specifying-dependencies.html:228: broken link - cargo/reference/guide/index.html
[01:20:02] cargo/reference/specifying-dependencies.html:276: broken link - cargo/reference/reference/manifest.html
[01:20:02] cargo/reference/specifying-dependencies.html:278: broken link - cargo/reference/reference/manifest.html
[01:20:02] cargo/reference/specifying-dependencies.html:427: broken link - cargo/reference/reference/config.html
[01:20:02] cargo/reference/specifying-dependencies.html:527: broken link - cargo/reference/reference/manifest.html
[01:20:02] cargo/reference/environment-variables.html:174: broken link - cargo/reference/reference/config.html
[01:20:02] cargo/reference/environment-variables.html:252: broken link - cargo/reference/reference/build-scripts.html
[01:20:02] cargo/reference/environment-variables.html:259: broken link - cargo/reference/reference/config.html
[01:20:02] embedded-book/print.html:377: broken link - embedded-book/install/linux.html
[01:20:02] embedded-book/print.html:378: broken link - embedded-book/install/windows.html
[01:20:02] embedded-book/print.html:379: broken link - embedded-book/install/macos.html
[01:20:02] embedded-book/print.html:482: broken link - embedded-book/verify.html
[01:20:02] embedded-book/print.html:494: broken link - embedded-book/verify.html
[01:20:02] embedded-book/print.html:520: broken link - embedded-book/verify.html
[01:20:02] embedded-book/print.html:555: broken link - hardware.html
[01:20:02] embedded-book/print.html:562: broken link - hardware.html
[01:20:02] embedded-book/print.html:565: broken link - embedded-book/linux.html
[01:20:02] embedded-book/print.html:631: broken link - embedded-book/hardware.html
[01:20:02] embedded-book/print.html:1110: broken link - embedded-book/qemu.html
[01:20:02] embedded-book/print.html:1154: broken link - intro/install/verify.html
[01:20:02] embedded-book/print.html:1176: broken link - intro/install/verify.html
[01:20:02] embedded-book/print.html:1332: broken link - portability/index.html
[01:20:02] embedded-book/print.html:1355: broken link - peripherals/index.html
[01:20:05] edition-guide/rust-2018/the-compiler/index.html:141: broken link - edition-guide/rust-2018/the-compiler/rust-2018/the-compiler/improved-error-messages.html
[01:20:05] edition-guide/rust-2018/macros/index.html:141: broken link - edition-guide/rust-2018/macros/rust-2018/macros/custom-derive.html
[01:20:05] edition-guide/rust-2018/cargo-and-crates-io/index.html:141: broken link - edition-guide/rust-2018/cargo-and-crates-io/rust-2018/cargo-and-crates-io/cargo-check-for-faster-checking.html
[01:20:05] edition-guide/rust-2018/documentation/index.html:141: broken link - edition-guide/rust-2018/documentation/rust-2018/documentation/new-editions-of-the-book.html
[01:20:05] edition-guide/rust-2018/control-flow/index.html:141: broken link - edition-guide/rust-2018/control-flow/rust-2018/control-flow/async-await-for-easier-concurrency.html
[01:20:05] edition-guide/rust-2018/module-system/path-clarity.html:217: broken link - edition-guide/rust-2018/module-system/rust-2018/macros/macro-changes.html
[01:20:05] edition-guide/rust-2018/module-system/index.html:141: broken link - edition-guide/rust-2018/module-system/rust-2018/module-system/path-clarity.html
[01:20:05] edition-guide/rust-2018/trait-system/index.html:141: broken link - edition-guide/rust-2018/trait-system/rust-2018/trait-system/impl-trait-for-returning-complex-types-with-ease.html
[01:20:05] edition-guide/rust-2018/trait-system/dyn-trait-for-trait-objects.html:172: broken link - edition-guide/rust-2018/trait-system/rust-2018/trait-system/impl-trait-for-returning-complex-types-with-ease.html
[01:20:05] edition-guide/rust-2018/platform-and-target-support/index.html:141: broken link - edition-guide/rust-2018/platform-and-target-support/rust-2018/platform-and-target-support/libcore-for-low-level-rust.html
[01:20:05] edition-guide/rust-2018/rustdoc/index.html:141: broken link - edition-guide/rust-2018/rustdoc/rust-2018/rustdoc/documentation-tests-can-now-compile-fail.html
[01:20:05] edition-guide/rust-2018/ownership-and-lifetimes/lifetime-elision-in-impl.html:182: broken link - edition-guide/rust-2018/ownership-and-lifetimes/rust-2018/ownership-and-lifetimes/the-anonymous-lifetime.html
[01:20:05] edition-guide/rust-2018/ownership-and-lifetimes/index.html:141: broken link - edition-guide/rust-2018/ownership-and-lifetimes/rust-2018/ownership-and-lifetimes/default-match-bindings.html
[01:20:05] edition-guide/rust-2018/data-types/index.html:141: broken link - edition-guide/rust-2018/data-types/rust-2018/data-types/field-init-shorthand.html
[01:20:05] edition-guide/rust-2018/error-handling-and-panics/index.html:141: broken link - edition-guide/rust-2018/error-handling-and-panics/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html
[01:20:05] edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html:160: broken link - edition-guide/editions/rust-2018/trait-system/no-anon-params.html
[01:20:05] edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html:212: broken link - edition-guide/editions/rust-2018/trait-system/dyn-trait-for-trait-objects.html
[01:20:07] unstable-book/language-features/asm.html:293: broken link - unstable-book/language-features/language-features/global-asm.html
[01:20:07] unstable-book/language-features/box-syntax.html:141: broken link - unstable-book/language-features/language-features/box-patterns.html
[01:20:07] unstable-book/language-features/unboxed-closures.html:141: broken link - unstable-book/language-features/library-features/fn-traits.html
[01:20:07] unstable-book/language-features/box-patterns.html:141: broken link - unstable-book/language-features/language-features/box-syntax.html
[01:20:07] unstable-book/language-features/non-ascii-idents.html:171: broken link - unstable-book/reference/keywords.html
[01:20:07] unstable-book/language-features/plugin.html:142: broken link - unstable-book/language-features/language-features/plugin-registrar.html
[01:20:07] unstable-book/language-features/plugin.html:164: broken link - unstable-book/book/macros.html
[01:20:07] unstable-book/language-features/plugin.html:248: broken link - unstable-book/reference/attributes.html
[01:20:07] unstable-book/language-features/plugin.html:278: broken link - unstable-book/reference/attributes.html
[01:20:07] unstable-book/language-features/plugin.html:351: broken link - unstable-book/reference/attributes.html
[01:20:07] unstable-book/language-features/lang-items.html:308: broken link - unstable-book/language-features/library-features/compiler-builtins-lib.html
[01:20:07] unstable-book/language-features/plugin-registrar.html:142: broken link - unstable-book/language-features/language-features/plugin.html
[01:20:07] unstable-book/language-features/global-asm.html:199: broken link - unstable-book/language-features/language-features/asm.html
[01:20:07] unstable-book/library-features/fn-traits.html:141: broken link - unstable-book/library-features/language-features/unboxed-closures.html
[01:20:07] reference/items/static-items.html:145: broken link - reference/items/identifiers.html
[01:20:07] reference/items/static-items.html:145: broken link - reference/items/types.html
[01:20:07] reference/items/static-items.html:146: broken link - reference/items/expressions.html
[01:20:07] reference/items/static-items.html:148: broken link - reference/items/items/constant-items.html
[01:20:07] reference/items/static-items.html:152: broken link - reference/items/interior-mutability.html
[01:20:07] reference/items/static-items.html:153: broken link - reference/items/destructors.html
[01:20:07] reference/items/static-items.html:158: broken link - reference/items/const_eval.html
[01:20:07] reference/items/type-aliases.html:145: broken link - reference/items/identifiers.html
[01:20:07] reference/items/type-aliases.html:145: broken link - reference/items/items/generics.html
[01:20:07] reference/items/type-aliases.html:146: broken link - reference/items/items/generics.html
[01:20:07] reference/items/type-aliases.html:146: broken link - reference/items/types.html
[01:20:07] reference/items/type-aliases.html:148: broken link - reference/items/types.html
[01:20:07] reference/items/traits.html:145: broken link - reference/items/identifiers.html
[01:20:07] reference/items/traits.html:146: broken link - reference/items/items/generics.html
[01:20:07] reference/items/traits.html:147: broken link - reference/items/trait-bounds.html
[01:20:07] reference/items/traits.html:148: broken link - reference/items/items/generics.html
[01:20:07] reference/items/traits.html:152: broken link - reference/items/attributes.html
[01:20:07] reference/items/traits.html:157: broken link - reference/items/macros.html
[01:20:07] reference/items/traits.html:160: broken link - reference/items/expressions/block-expr.html
[01:20:07] reference/items/traits.html:162: broken link - reference/items/expressions/block-expr.html
[01:20:07] reference/items/traits.html:164: broken link - reference/items/items/functions.html
[01:20:07] reference/items/traits.html:164: broken link - reference/items/identifiers.html
[01:20:07] reference/items/traits.html:164: broken link - reference/items/items/generics.html
[01:20:07] reference/items/traits.html:166: broken link - reference/items/items/functions.html
[01:20:07] reference/items/traits.html:166: broken link - reference/items/items/generics.html
[01:20:07] reference/items/traits.html:168: broken link - reference/items/items/functions.html
[01:20:07] reference/items/traits.html:168: broken link - reference/items/identifiers.html
[01:20:07] reference/items/traits.html:168: broken link - reference/items/items/generics.html
[01:20:07] reference/items/traits.html:169: broken link - reference/items/items/associated-items.html
[01:20:07] reference/items/traits.html:170: broken link - reference/items/items/functions.html
[01:20:07] reference/items/traits.html:170: broken link - reference/items/items/generics.html
[01:20:07] reference/items/traits.html:174: broken link - reference/items/patterns.html
[01:20:07] reference/items/traits.html:174: broken link - reference/items/types.html
[01:20:07] reference/items/traits.html:176: broken link - reference/items/identifiers.html
[01:20:07] reference/items/traits.html:176: broken link - reference/items/types.html
[01:20:07] reference/items/traits.html:176: broken link - reference/items/expressions.html
[01:20:07] reference/items/traits.html:178: broken link - reference/items/identifiers.html
[01:20:07] reference/items/traits.html:178: broken link - reference/items/trait-bounds.html
[01:20:07] reference/items/traits.html:181: broken link - reference/items/items/associated-items.html
[01:20:07] reference/items/traits.html:183: broken link - reference/items/items/associated-items.html
[01:20:07] reference/items/traits.html:184: broken link - reference/items/items/associated-items.html
[01:20:07] reference/items/traits.html:185: broken link - reference/items/items/associated-items.html
[01:20:07] reference/items/traits.html:190: broken link - reference/items/items/generics.html
[01:20:07] reference/items/traits.html:191: broken link - reference/items/items/implementations.html
[01:20:07] reference/items/traits.html:197: broken link - reference/items/trait-bounds.html
[01:20:07] reference/items/traits.html:200: broken link - reference/items/items/functions.html
[01:20:07] reference/items/traits.html:211: broken link - reference/items/types/trait-object.html
[01:20:07] reference/items/traits.html:219: broken link - reference/items/items/associated-items.html
[01:20:07] reference/items/traits.html:227: broken link - reference/items/items/generics.html
[01:20:07] reference/items/traits.html:227: broken link - reference/items/types/trait-object.html
[01:20:07] reference/items/traits.html:240: broken link - reference/items/items/generics.html
[01:20:07] reference/items/traits.html:289: broken link - reference/items/unsafety.html
[01:20:07] reference/items/traits.html:290: broken link - reference/items/items/implementations.html
[01:20:07] reference/items/traits.html:291: broken link - reference/items/special-types-and-traits.html
[01:20:07] reference/items/traits.html:291: broken link - reference/items/special-types-and-traits.html
[01:20:07] reference/items/traits.html:293: broken link - reference/items/identifiers.html
[01:20:07] reference/items/traits.html:294: broken link - reference/items/patterns.html
[01:20:07] reference/items/traits.html:294: broken link - reference/items/identifiers.html
[01:20:07] reference/items/traits.html:308: broken link - reference/items/identifiers.html
[01:20:07] reference/items/traits.html:309: broken link - reference/items/identifiers.html
[01:20:07] reference/items/traits.html:310: broken link - reference/items/patterns.html
[01:20:07] reference/items/traits.html:311: broken link - reference/items/identifiers.html
[01:20:07] reference/items/traits.html:312: broken link - reference/items/identifiers.html
[01:20:07] reference/items/modules.html:145: broken link - reference/items/identifiers.html
[01:20:07] reference/items/modules.html:146: broken link - reference/items/identifiers.html
[01:20:07] reference/items/modules.html:147: broken link - reference/items/attributes.html
[01:20:07] reference/items/modules.html:148: broken link - reference/items/items.html
[01:20:07] reference/items/modules.html:151: broken link - reference/items/items.html
[01:20:07] reference/items/modules.html:182: broken link - reference/items/paths.html
[01:20:07] reference/items/modules.html:247: broken link - reference/items/crates-and-source-files.html
[01:20:07] reference/items/modules.html:249: broken link - reference/items/crates-and-source-files.html
[01:20:07] reference/items/modules.html:250: broken link - reference/items/attributes.html
[01:20:07] reference/items/modules.html:255: broken link - reference/items/conditional-compilation.html
[01:20:07] reference/items/modules.html:256: broken link - reference/items/attributes.html
[01:20:07] reference/items/modules.html:256: broken link - reference/items/attributes.html
[01:20:07] reference/items/modules.html:256: broken link - reference/items/attributes.html
[01:20:07] reference/items/enumerations.html:146: broken link - reference/items/identifiers.html
[01:20:07] reference/items/enumerations.html:147: broken link - reference/items/items/generics.html
[01:20:07] reference/items/enumerations.html:148: broken link - reference/items/items/generics.html
[01:20:07] reference/items/enumerations.html:154: broken link - reference/items/identifiers.html
[01:20:07] reference/items/enumerations.html:157: broken link - reference/items/items/structs.html
[01:20:07] reference/items/enumerations.html:159: broken link - reference/items/items/structs.html
[01:20:07] reference/items/enumerations.html:161: broken link - reference/items/expressions.html
[01:20:07] reference/items/enumerations.html:164: broken link - reference/items/types/enum.html
[01:20:07] reference/items/enumerations.html:195: broken link - reference/std/mem/fn.discriminant.html
[01:20:07] reference/items/enumerations.html:200: broken link - reference/items/expressions/operator-expr.html
[01:20:07] reference/items/enumerations.html:201: broken link - reference/items/const_eval.html
[01:20:07] reference/items/enumerations.html:217: broken link - reference/items/type-layout.html
[01:20:07] reference/items/enumerations.html:220: broken link - reference/items/type-layout.html
[01:20:07] reference/items/enumerations.html:220: broken link - reference/items/type-layout.html
[01:20:07] reference/items/functions.html:145: broken link - reference/items/identifiers.html
[01:20:07] reference/items/functions.html:145: broken link - reference/items/items/generics.html
[01:20:07] reference/items/functions.html:147: broken link - reference/items/items/generics.html
[01:20:07] reference/items/functions.html:148: broken link - reference/items/expressions/block-expr.html
[01:20:07] reference/items/functions.html:152: broken link - reference/items/tokens.html
[01:20:07] reference/items/functions.html:152: broken link - reference/items/tokens.html
[01:20:07] reference/items/functions.html:156: broken link - reference/items/patterns.html
[01:20:07] reference/items/functions.html:156: broken link - reference/items/types.html
[01:20:07] reference/items/functions.html:158: broken link - reference/items/types.html
[01:20:07] reference/items/functions.html:160: broken link - reference/items/expressions/block-expr.html
[01:20:07] reference/items/functions.html:162: broken link - reference/items/variables.html
[01:20:07] reference/items/functions.html:164: broken link - reference/items/types.html
[01:20:07] reference/items/functions.html:167: broken link - reference/items/types/function-item.html
[01:20:07] reference/items/functions.html:177: broken link - reference/items/patterns.html
[01:20:07] reference/items/functions.html:209: broken link - reference/items/items/traits.html
[01:20:07] reference/items/functions.html:233: broken link - reference/items/paths.html
[01:20:07] reference/items/functions.html:239: broken link - reference/items/items/external-blocks.html
[01:20:07] reference/items/functions.html:267: broken link - reference/items/const_eval.html
[01:20:07] reference/items/functions.html:290: broken link - reference/items/trait-bounds.html
[01:20:07] reference/items/functions.html:294: broken link - reference/items/trait-bounds.html
[01:20:07] reference/items/functions.html:336: broken link - reference/items/attributes.html
[01:20:07] reference/items/functions.html:336: broken link - reference/items/attributes.html
[01:20:07] reference/items/functions.html:337: broken link - reference/items/expressions/block-expr.html
[01:20:07] reference/items/functions.html:348: broken link - reference/items/conditional-compilation.html
[01:20:07] reference/items/functions.html:348: broken link - reference/items/attributes.html
[01:20:07] reference/items/functions.html:349: broken link - reference/items/attributes.html
[01:20:07] reference/items/functions.html:349: broken link - reference/items/attributes.html
[01:20:07] reference/items/functions.html:350: broken link - reference/items/attributes.html
[01:20:07] reference/items/functions.html:350: broken link - reference/items/procedural-macros.html
[01:20:07] reference/items/functions.html:350: broken link - reference/items/attributes.html
[01:20:07] reference/items/functions.html:351: broken link - reference/items/attributes.html
[01:20:07] reference/items/structs.html:149: broken link - reference/items/identifiers.html
[01:20:07] reference/items/structs.html:150: broken link - reference/items/items/generics.html
[01:20:07] reference/items/structs.html:151: broken link - reference/items/items/generics.html
[01:20:07] reference/items/structs.html:155: broken link - reference/items/identifiers.html
[01:20:07] reference/items/structs.html:156: broken link - reference/items/items/generics.html
[01:20:07] reference/items/structs.html:158: broken link - reference/items/items/generics.html
[01:20:07] reference/items/structs.html:163: broken link - reference/items/attributes.html
[01:20:07] reference/items/structs.html:164: broken link - reference/items/visibility-and-privacy.html
[01:20:07] reference/items/structs.html:165: broken link - reference/items/identifiers.html
[01:20:07] reference/items/structs.html:165: broken link - reference/items/types.html
[01:20:07] reference/items/structs.html:169: broken link - reference/items/attributes.html
[01:20:07] reference/items/structs.html:170: broken link - reference/items/visibility-and-privacy.html
[01:20:07] reference/items/structs.html:171: broken link - reference/items/types.html
[01:20:07] reference/items/structs.html:173: broken link - reference/items/types/struct.html
[01:20:07] reference/items/structs.html:182: broken link - reference/items/types/tuple.html
[01:20:07] reference/items/structs.html:209: broken link - reference/items/type-layout.html
[01:20:07] reference/items/implementations.html:147: broken link - reference/items/items/generics.html
[01:20:07] reference/items/implementations.html:147: broken link - reference/items/types.html
[01:20:07] reference/items/implementations.html:147: broken link - reference/items/items/generics.html
[01:20:07] reference/items/implementations.html:148: broken link - reference/items/attributes.html
[01:20:07] reference/items/implementations.html:152: broken link - reference/items/attributes.html
[01:20:07] reference/items/implementations.html:153: broken link - reference/items/macros.html
[01:20:07] reference/items/implementations.html:154: broken link - reference/items/visibility-and-privacy.html
[01:20:07] reference/items/implementations.html:154: broken link - reference/items/items/constant-items.html
[01:20:07] reference/items/implementations.html:154: broken link - reference/items/items/functions.html
[01:20:07] reference/items/implementations.html:154: broken link - reference/items/items/associated-items.html
[01:20:07] reference/items/implementations.html:157: broken link - reference/items/items/generics.html
[01:20:07] reference/items/implementations.html:158: broken link - reference/items/paths.html
[01:20:07] reference/items/implementations.html:158: broken link - reference/items/types.html
[01:20:07] reference/items/implementations.html:159: broken link - reference/items/items/generics.html
[01:20:07] reference/items/implementations.html:161: broken link - reference/items/attributes.html
[01:20:07] reference/items/implementations.html:165: broken link - reference/items/attributes.html
[01:20:07] reference/items/implementations.html:166: broken link - reference/items/macros.html
[01:20:07] reference/items/implementations.html:167: broken link - reference/items/visibility-and-privacy.html
[01:20:07] reference/items/implementations.html:167: broken link - reference/items/items/type-aliases.html
[01:20:07] reference/items/implementations.html:167: broken link - reference/items/items/constant-items.html
[01:20:07] reference/items/implementations.html:167: broken link - reference/items/items/functions.html
[01:20:07] reference/items/implementations.html:167: broken link - reference/items/items/associated-items.html
[01:20:07] reference/items/implementations.html:177: broken link - reference/items/items/traits.html
[01:20:07] reference/items/implementations.html:207: broken link - reference/items/items/traits.html
[01:20:07] reference/items/implementations.html:218: broken link - reference/items/items/traits.html
[01:20:07] reference/items/implementations.html:295: broken link - reference/items/attributes.html
[01:20:07] reference/items/implementations.html:296: broken link - reference/items/attributes.html
[01:20:07] reference/items/implementations.html:298: broken link - reference/items/conditional-compilation.html
[01:20:07] reference/items/implementations.html:298: broken link - reference/items/attributes.html
[01:20:07] reference/items/implementations.html:298: broken link - reference/items/attributes.html
[01:20:07] reference/items/implementations.html:298: broken link - reference/items/attributes.html
[01:20:07] reference/items/associated-items.html:142: broken link - reference/items/items/traits.html
[01:20:07] reference/items/associated-items.html:143: broken link - reference/items/items/implementations.html
[01:20:07] reference/items/associated-items.html:156: broken link - reference/items/items/functions.html
[01:20:07] reference/items/associated-items.html:164: broken link - reference/items/types/function-item.html
[01:20:07] reference/items/associated-items.html:184: broken link - reference/items/paths.html
[01:20:07] reference/items/associated-items.html:206: broken link - reference/items/items/functions.html
[01:20:07] reference/items/associated-items.html:206: broken link - reference/items/identifiers.html
[01:20:07] reference/items/associated-items.html:206: broken link - reference/items/items/generics.html
[01:20:07] reference/items/associated-items.html:207: broken link - reference/items/items/functions.html
[01:20:07] reference/items/associated-items.html:208: broken link - reference/items/items/functions.html
[01:20:07] reference/items/associated-items.html:208: broken link - reference/items/items/generics.html
[01:20:07] reference/items/associated-items.html:209: broken link - reference/items/expressions/block-expr.html
[01:20:07] reference/items/associated-items.html:211: broken link - reference/items/trait-bounds.html
[01:20:07] reference/items/associated-items.html:212: broken link - reference/items/types.html
[01:20:07] reference/items/associated-items.html:215: broken link - reference/items/expressions/method-call-expr.html
[01:20:07] reference/items/associated-items.html:241: broken link - reference/items/items/implementations.html
[01:20:07] reference/items/associated-items.html:277: broken link - reference/items/items/type-aliases.html
[01:20:07] reference/items/associated-items.html:278: broken link - reference/items/items/implementations.html
[01:20:07] reference/items/associated-items.html:281: broken link - reference/items/identifiers.html
[01:20:07] reference/items/associated-items.html:286: broken link - reference/items/identifiers.html
[01:20:07] reference/items/associated-items.html:286: broken link - reference/items/types.html
[01:20:07] reference/items/associated-items.html:346: broken link - reference/items/items/constant-items.html
[01:20:07] reference/items/associated-items.html:353: broken link - reference/items/items/constant-items.html
[01:20:07] reference/items/extern-crates.html:145: broken link - reference/items/identifiers.html
[01:20:07] reference/items/extern-crates.html:145: broken link - reference/items/identifiers.html
[01:20:07] reference/items/extern-crates.html:148: broken link - reference/items/identifiers.html
[01:20:07] reference/items/extern-crates.html:156: broken link - reference/items/identifiers.html
[01:20:07] reference/items/extern-crates.html:178: broken link - reference/items/items/use-declarations.html
[01:20:07] reference/items/extern-crates.html:180: broken link - reference/items/items/use-declarations.html
[01:20:07] reference/items/extern-crates.html:188: broken link - reference/items/paths.html
[01:20:07] reference/items/extern-crates.html:188: broken link - reference/items/paths.html
[01:20:07] reference/items/use-declarations.html:147: broken link - reference/items/paths.html
[01:20:07] reference/items/use-declarations.html:148: broken link - reference/items/paths.html
[01:20:07] reference/items/use-declarations.html:149: broken link - reference/items/paths.html
[01:20:07] reference/items/use-declarations.html:149: broken link - reference/items/identifiers.html
[01:20:07] reference/items/use-declarations.html:152: broken link - reference/items/paths.html
[01:20:07] reference/items/use-declarations.html:153: broken link - reference/items/items/modules.html
[01:20:07] reference/items/use-declarations.html:154: broken link - reference/items/expressions/block-expr.html
[01:20:07] reference/items/use-declarations.html:157: broken link - reference/items/items/extern-crates.html
[01:20:07] reference/items/use-declarations.html:213: broken link - reference/items/paths.html
[01:20:07] reference/items/generics.html:152: broken link - reference/items/attributes.html
[01:20:07] reference/items/generics.html:152: broken link - reference/items/tokens.html
[01:20:07] reference/items/generics.html:152: broken link - reference/items/trait-bounds.html
[01:20:07] reference/items/generics.html:156: broken link - reference/items/attributes.html
[01:20:07] reference/items/generics.html:156: broken link - reference/items/identifiers.html
[01:20:07] reference/items/generics.html:156: broken link - reference/items/trait-bounds.html
[01:20:07] reference/items/generics.html:156: broken link - reference/items/types.html
[01:20:07] reference/items/generics.html:172: broken link - reference/items/types/pointer.html
[01:20:07] reference/items/generics.html:172: broken link - reference/items/types/pointer.html
[01:20:07] reference/items/generics.html:172: broken link - reference/items/types/array.html
[01:20:07] reference/items/generics.html:172: broken link - reference/items/types/array.html
[01:20:07] reference/items/generics.html:172: broken link - reference/items/types/tuple.html
[01:20:07] reference/items/generics.html:173: broken link - reference/items/types/function-pointer.html
[01:20:07] reference/items/generics.html:184: broken link - reference/items/trait-bounds.html
[01:20:07] reference/items/generics.html:184: broken link - reference/items/trait-bounds.html
[01:20:07] reference/items/generics.html:186: broken link - reference/items/types.html
[01:20:07] reference/items/generics.html:186: broken link - reference/items/trait-bounds.html
[01:20:07] reference/items/generics.html:195: broken link - reference/items/special-types-and-traits.html
[01:20:07] reference/items/generics.html:195: broken link - reference/items/special-types-and-traits.html
[01:20:07] reference/items/generics.html:195: broken link - reference/items/special-types-and-traits.html
[01:20:07] reference/items/generics.html:197: broken link - reference/items/types/trait-object.html
[01:20:07] reference/items/generics.html:197: broken link - reference/items/types/array.html
[01:20:07] reference/items/generics.html:212: broken link - reference/items/attributes.html
[01:20:07] reference/items/unions.html:145: broken link - reference/items/identifiers.html
[01:20:07] reference/items/unions.html:145: broken link - reference/items/items/generics.html
[01:20:07] reference/items/unions.html:145: broken link - reference/items/items/generics.html
[01:20:07] reference/items/unions.html:146: broken link - reference/items/items/structs.html
[01:20:07] reference/items/external-blocks.html:145: broken link - reference/items/items/functions.html
[01:20:07] reference/items/external-blocks.html:146: broken link - reference/items/attributes.html
[01:20:07] reference/items/external-blocks.html:150: broken link - reference/items/attributes.html
[01:20:07] reference/items/external-blocks.html:151: broken link - reference/items/visibility-and-privacy.html
[01:20:07] reference/items/external-blocks.html:154: broken link - reference/items/identifiers.html
[01:20:07] reference/items/external-blocks.html:154: broken link - reference/items/types.html
[01:20:07] reference/items/external-blocks.html:156: broken link - reference/items/identifiers.html
[01:20:07] reference/items/external-blocks.html:156: broken link - reference/items/items/generics.html
[01:20:07] reference/items/external-blocks.html:158: broken link - reference/items/items/functions.html
[01:20:07] reference/items/external-blocks.html:158: broken link - reference/items/items/generics.html
[01:20:07] reference/items/external-blocks.html:162: broken link - reference/items/identifiers.html
[01:20:07] reference/items/external-blocks.html:162: broken link - reference/items/types.html
[01:20:07] reference/items/external-blocks.html:172: broken link - reference/items/identifiers.html
[01:20:07] reference/items/external-blocks.html:182: broken link - reference/items/attributes.html
[01:20:07] reference/items/constant-items.html:145: broken link - reference/items/identifiers.html
[01:20:07] reference/items/constant-items.html:145: broken link - reference/items/types.html
[01:20:07] reference/items/constant-items.html:145: broken link - reference/items/expressions.html
[01:20:07] reference/items/constant-items.html:147: broken link - reference/items/const_eval.html
[01:20:07] reference/items/constant-items.html:156: broken link - reference/items/lifetime-elision.html
[01:20:07] reference/expressions/operator-expr.html:177: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:178: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:181: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:188: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:188: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:222: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:225: broken link - reference/expressions/types/pointer.html
[01:20:07] reference/expressions/operator-expr.html:227: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:231: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:246: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:292: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:293: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:299: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:318: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:318: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:319: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:319: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:320: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:320: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:321: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:321: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:322: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:322: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:323: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:323: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:324: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:324: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:325: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:325: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:326: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:326: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:327: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:327: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:333: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:369: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:369: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:370: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:370: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:371: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:371: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:372: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:372: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:373: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:373: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:374: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:374: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:387: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:416: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:416: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:417: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:417: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:436: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:436: broken link - reference/expressions/types.html
[01:20:07] reference/expressions/operator-expr.html:453: broken link - reference/expressions/type-coercions.html
[01:20:07] reference/expressions/operator-expr.html:465: broken link - reference/expressions/types/function-pointer.html
[01:20:07] reference/expressions/operator-expr.html:528: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:528: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:530: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:531: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:532: broken link - reference/expressions/types/tuple.html
[01:20:07] reference/expressions/operator-expr.html:533: broken link - reference/expressions/destructors.html
[01:20:07] reference/expressions/operator-expr.html:535: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:550: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:550: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:551: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:551: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:552: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:552: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:553: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:553: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:554: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:554: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:555: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:555: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:556: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:556: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:557: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:557: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:558: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:558: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:559: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:559: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/operator-expr.html:564: broken link - reference/expressions/types/tuple.html
[01:20:07] reference/expressions/operator-expr.html:567: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/if-expr.html:146: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/if-expr.html:146: broken link - reference/expressions/expressions/block-expr.html
[01:20:07] reference/expressions/if-expr.html:148: broken link - reference/expressions/expressions/block-expr.html
[01:20:07] reference/expressions/if-expr.html:185: broken link - reference/expressions/patterns.html
[01:20:07] reference/expressions/if-expr.html:185: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/if-expr.html:186: broken link - reference/expressions/expressions/block-expr.html
[01:20:07] reference/expressions/if-expr.html:188: broken link - reference/expressions/expressions/block-expr.html
[01:20:07] reference/expressions/if-expr.html:246: broken link - reference/expressions/expressions/operator-expr.html
[01:20:07] reference/expressions/loop-expr.html:167: broken link - reference/expressions/expressions/block-expr.html
[01:20:07] reference/expressions/loop-expr.html:172: broken link - reference/expressions/types/never.html
[01:20:07] reference/expressions/loop-expr.html:179: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/loop-expr.html:179: broken link - reference/expressions/expressions/block-expr.html
[01:20:07] reference/expressions/loop-expr.html:200: broken link - reference/expressions/patterns.html
[01:20:07] reference/expressions/loop-expr.html:200: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/loop-expr.html:201: broken link - reference/expressions/expressions/block-expr.html
[01:20:07] reference/expressions/loop-expr.html:236: broken link - reference/expressions/patterns.html
[01:20:07] reference/expressions/loop-expr.html:236: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/loop-expr.html:237: broken link - reference/expressions/expressions/block-expr.html
[01:20:07] reference/expressions/loop-expr.html:291: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/loop-expr.html:299: broken link - reference/expressions/tokens.html
[01:20:07] reference/expressions/loop-expr.html:312: broken link - reference/expressions/tokens.html
[01:20:07] reference/expressions/loop-expr.html:312: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/loop-expr.html:347: broken link - reference/expressions/tokens.html
[01:20:07] reference/expressions/struct-expr.html:149: broken link - reference/expressions/paths.html
[01:20:07] reference/expressions/struct-expr.html:149: broken link - reference/expressions/attributes.html
[01:20:07] reference/expressions/struct-expr.html:153: broken link - reference/expressions/identifiers.html
[01:20:07] reference/expressions/struct-expr.html:154: broken link - reference/expressions/identifiers.html
[01:20:07] reference/expressions/struct-expr.html:154: broken link - reference/expressions/tokens.html
[01:20:07] reference/expressions/struct-expr.html:154: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/struct-expr.html:156: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/struct-expr.html:158: broken link - reference/expressions/paths.html
[01:20:07] reference/expressions/struct-expr.html:159: broken link - reference/expressions/attributes.html
[01:20:07] reference/expressions/struct-expr.html:160: broken link - reference/expressions/expressions.html
[01:20:07] reference/expressions/struct-expr.html:160: broken link - reference/expressions/expressions.html
---
travis_time:end:005fe3b2:start=1544479623887270472,finish=1544479623895043057,duration=7772585
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:109f749a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cad6efc
travis_time:start:0cad6efc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10b0b52c
$ dmesg | grep -i kill
