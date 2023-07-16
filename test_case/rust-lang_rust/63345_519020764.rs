plain
2019-08-07T08:08:01.4293793Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-07T08:08:01.4293846Z 
2019-08-07T08:08:01.4294112Z   git checkout -b <new-branch-name>
2019-08-07T08:08:01.4294159Z 
2019-08-07T08:08:01.4295782Z HEAD is now at ae515c8b6 Auto merge of #63345 - Centril:rollup-9ue8fz6, r=Centril
2019-08-07T08:08:01.4477302Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-07T08:08:01.4480468Z ==============================================================================
2019-08-07T08:08:01.4480711Z Task         : Bash
2019-08-07T08:08:01.4480772Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-07T08:11:31.1388449Z checking whether compiler accepts -fno-unwind-tables... yes
2019-08-07T08:11:31.1490415Z checking whether compiler accepts -fno-asynchronous-unwind-tables... yes
2019-08-07T08:11:31.1586060Z checking whether compiler accepts -ffunction-sections... yes
2019-08-07T08:11:31.1679018Z checking whether compiler accepts -fdata-sections... yes
2019-08-07T08:11:31.1712962Z checking whether linker accepts -march=i486... no
2019-08-07T08:11:31.1839394Z checking whether linker accepts -mtune=generic... yes
2019-08-07T08:11:31.2033296Z checking whether compiler accepts -Werror=implicit-int... yes
2019-08-07T08:11:31.2127886Z checking whether compiler accepts -Werror=pointer-sign... yes
2019-08-07T08:11:31.2230372Z checking whether compiler accepts -Werror=pointer-arith... yes
2019-08-07T08:11:31.2281760Z checking preprocessor condition __PIC__... true
---
2019-08-07T08:11:31.3958067Z checking preprocessor condition __FAST_MATH__... false
2019-08-07T08:11:31.4055903Z creating config.mak... done
2019-08-07T08:11:31.4067788Z + '[' i686 = i586 -o i686 = i686 ']'
2019-08-07T08:11:31.4072436Z ++ nproc
2019-08-07T08:11:31.4079764Z + hide_output make -j2 AR=ar RANLIB=ranlib
2019-08-07T08:11:50.1119850Z musl.sh: line 3:   174 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-08-07T08:11:50.1120068Z + hide_output make install
2019-08-07T08:11:50.1120163Z + set +x
2019-08-07T08:11:51.4915804Z musl.sh: line 3:  4305 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
---
2019-08-07T08:11:51.8209216Z checking whether compiler accepts -fno-unwind-tables... yes
2019-08-07T08:11:51.8316234Z checking whether compiler accepts -fno-asynchronous-unwind-tables... yes
2019-08-07T08:11:51.8426289Z checking whether compiler accepts -ffunction-sections... yes
2019-08-07T08:11:51.8541046Z checking whether compiler accepts -fdata-sections... yes
2019-08-07T08:11:51.8664870Z checking whether linker accepts -mtune=generic... yes
2019-08-07T08:11:51.8855464Z checking whether compiler accepts -Werror=implicit-int... yes
2019-08-07T08:11:51.8960269Z checking whether compiler accepts -Werror=pointer-sign... yes
2019-08-07T08:11:51.9055811Z checking whether compiler accepts -Werror=pointer-arith... yes
2019-08-07T08:11:51.9109437Z checking preprocessor condition __PIC__... true
---
2019-08-07T08:11:52.0686802Z checking preprocessor condition __FAST_MATH__... false
2019-08-07T08:11:52.0787688Z creating config.mak... done
2019-08-07T08:11:52.0798619Z + '[' i586 = i586 -o i586 = i686 ']'
2019-08-07T08:11:52.0828356Z ++ nproc
2019-08-07T08:11:52.0828625Z + hide_output make -j2 AR=ar RANLIB=ranlib
2019-08-07T08:12:10.6097550Z musl.sh: line 3:  5873 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-08-07T08:12:10.6099383Z + hide_output make install
2019-08-07T08:12:10.6099767Z + set +x
2019-08-07T08:12:11.9600177Z musl.sh: line 3: 10006 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
---
2019-08-07T09:19:05.0001450Z test [ui] ui/async-await/await-keyword/post_expansion_error.rs ... ok
2019-08-07T09:19:05.0692047Z test [ui] ui/async-await/await-unsize.rs ... ok
2019-08-07T09:19:05.0715172Z test [ui] ui/async-await/await-keyword/incorrect-syntax-suggestions.rs ... ok
2019-08-07T09:19:05.1244389Z test [ui] ui/async-await/bound-normalization.rs ... ok
2019-08-07T09:19:05.1479339Z test [ui] ui/async-await/conditional-and-guaranteed-initialization.rs ... ok
2019-08-07T09:19:05.2655088Z test [ui] ui/async-await/dont-suggest-missing-await.rs ... ok
2019-08-07T09:19:08.0599148Z test [ui] ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs ... ok
2019-08-07T09:19:08.2225200Z test [ui] ui/async-await/drop-order/drop-order-for-async-fn-parameters.rs ... ok
2019-08-07T09:19:09.0630039Z test [ui] ui/async-await/drop-order/drop-order-locals-are-hidden.rs ... ok
---
2019-08-07T09:31:26.2317594Z    Compiling libc v0.2.60
2019-08-07T09:31:26.2509078Z    Compiling compiler_builtins v0.1.18
2019-08-07T09:31:31.4941891Z    Compiling backtrace-sys v0.1.30
2019-08-07T09:31:33.3554962Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-08-07T09:31:33.3555404Z error: failed to run custom build command for `unwind v0.0.0 (/checkout/src/libunwind)`
2019-08-07T09:31:33.3555915Z Caused by:
2019-08-07T09:31:33.3555915Z Caused by:
2019-08-07T09:31:33.3556485Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/unwind-a04f9435c6eb75e3/build-script-build` (exit code: 101)
2019-08-07T09:31:33.3556711Z --- stdout
2019-08-07T09:31:33.3556938Z cargo:rerun-if-changed=build.rs
2019-08-07T09:31:33.3557262Z cargo:rustc-link-search=native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/build/unwind-3b3aa3b1105b96b5/out
2019-08-07T09:31:33.3558000Z running: "musl-g++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m32" "-static" "-march=i686" "-Wl,-melf_i386" "-I" "../llvm-project/libunwind/include" "-std=c99" "-std=c++11" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/build/unwind-3b3aa3b1105b96b5/out/../llvm-project/libunwind/src/Unwind-EHABI.o" "-c" "../llvm-project/libunwind/src/Unwind-EHABI.cpp"
2019-08-07T09:31:33.3558842Z --- stderr
2019-08-07T09:31:33.3559052Z thread 'main' panicked at '
2019-08-07T09:31:33.3559217Z 
2019-08-07T09:31:33.3559217Z 
2019-08-07T09:31:33.3559475Z Internal error occurred: Failed to find tool. Is `musl-g++` installed?
2019-08-07T09:31:33.3559806Z ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.35/src/lib.rs:2398:5
2019-08-07T09:31:33.3559909Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-07T09:31:33.3559955Z 
2019-08-07T09:31:33.3560216Z warning: build failed, waiting for other jobs to finish...
2019-08-07T09:31:33.3560216Z warning: build failed, waiting for other jobs to finish...
2019-08-07T09:31:55.3393280Z [RUSTC-TIMING] core test:false 29.108
2019-08-07T09:31:55.3457073Z error: build failed
2019-08-07T09:31:55.3480035Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i686-unknown-linux-musl" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-07T09:31:55.3490584Z expected success, got: exit code: 101
2019-08-07T09:31:55.3491089Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-08-07T09:31:55.3491180Z Build completed unsuccessfully in 1:16:31
2019-08-07T09:31:57.2078119Z ##[error]Bash exited with code '1'.
2019-08-07T09:31:57.2126489Z ##[section]Starting: Upload CPU usage statistics
2019-08-07T09:31:57.2138080Z ==============================================================================
2019-08-07T09:31:57.2138184Z Task         : Bash
2019-08-07T09:31:57.2138242Z Description  : Run a Bash script on macOS, Linux, or Windows
