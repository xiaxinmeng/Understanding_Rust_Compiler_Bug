
$ cd library/backtrace
$ git status
HEAD detached at 4083a90
Changes not staged for commit:
  (use "git add/rm <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
	deleted:    .github/workflows/main.yml
	modified:   .gitignore
	modified:   .gitmodules
	modified:   Cargo.toml
	modified:   LICENSE-APACHE
	modified:   LICENSE-MIT
	modified:   README.md
	deleted:    benches/benchmarks.rs
	deleted:    ci/android-ndk.sh
	deleted:    ci/android-sdk.sh
	deleted:    ci/docker/aarch64-linux-android/Dockerfile
	deleted:    ci/docker/aarch64-unknown-linux-gnu/Dockerfile
	deleted:    ci/docker/arm-linux-androideabi/Dockerfile
	deleted:    ci/docker/arm-unknown-linux-gnueabihf/Dockerfile
	deleted:    ci/docker/armv7-linux-androideabi/Dockerfile
	deleted:    ci/docker/armv7-unknown-linux-gnueabihf/Dockerfile
	deleted:    ci/docker/i586-unknown-linux-gnu/Dockerfile
	deleted:    ci/docker/i686-linux-android/Dockerfile
	deleted:    ci/docker/i686-unknown-linux-gnu/Dockerfile
	deleted:    ci/docker/powerpc64-unknown-linux-gnu/Dockerfile
	deleted:    ci/docker/x86_64-linux-android/Dockerfile
	deleted:    ci/docker/x86_64-pc-windows-gnu/Dockerfile
	deleted:    ci/docker/x86_64-unknown-linux-gnu/Dockerfile
	deleted:    ci/docker/x86_64-unknown-linux-musl/Dockerfile
	deleted:    ci/run-docker.sh
	deleted:    ci/run.sh
	deleted:    ci/runtest-android.rs
	deleted:    crates/as-if-std/Cargo.toml
	deleted:    crates/as-if-std/build.rs
	deleted:    crates/as-if-std/src/lib.rs
	deleted:    crates/backtrace-sys/Cargo.toml
	deleted:    crates/backtrace-sys/LICENSE-APACHE
	deleted:    crates/backtrace-sys/LICENSE-MIT
	deleted:    crates/backtrace-sys/build.rs
	deleted:    crates/backtrace-sys/src/android-api.c
	deleted:    crates/backtrace-sys/src/lib.rs
	deleted:    crates/cpp_smoke_test/Cargo.toml
	deleted:    crates/cpp_smoke_test/build.rs
	deleted:    crates/cpp_smoke_test/cpp/trampoline.cpp
	deleted:    crates/cpp_smoke_test/src/lib.rs
	deleted:    crates/cpp_smoke_test/tests/smoke.rs
	deleted:    crates/dylib-dep/Cargo.toml
	deleted:    crates/dylib-dep/src/lib.rs
	deleted:    crates/line-tables-only/Cargo.toml
	deleted:    crates/line-tables-only/build.rs
	deleted:    crates/line-tables-only/src/callback.c
	deleted:    crates/line-tables-only/src/lib.rs
	deleted:    crates/macos_frames_test/Cargo.toml
	deleted:    crates/macos_frames_test/src/lib.rs
	deleted:    crates/macos_frames_test/tests/main.rs
	deleted:    crates/without_debuginfo/Cargo.toml
	deleted:    crates/without_debuginfo/src/lib.rs
	deleted:    crates/without_debuginfo/tests/smoke.rs
	deleted:    examples/backtrace.rs
	deleted:    examples/raw.rs
	deleted:    src/backtrace/dbghelp.rs
	deleted:    src/backtrace/libunwind.rs
	deleted:    src/backtrace/mod.rs
	deleted:    src/backtrace/noop.rs
	deleted:    src/capture.rs
	deleted:    src/dbghelp.rs
	deleted:    src/lib.rs
	deleted:    src/print.rs
	deleted:    src/print/fuchsia.rs
	deleted:    src/symbolize/dbghelp.rs
	deleted:    src/symbolize/gimli.rs
	deleted:    src/symbolize/gimli/coff.rs
	deleted:    src/symbolize/gimli/elf.rs
	deleted:    src/symbolize/gimli/macho.rs
	deleted:    src/symbolize/gimli/mmap_fake.rs
	deleted:    src/symbolize/gimli/mmap_unix.rs
	deleted:    src/symbolize/gimli/mmap_windows.rs
	deleted:    src/symbolize/gimli/stash.rs
	deleted:    src/symbolize/libbacktrace.rs
	deleted:    src/symbolize/mod.rs
	deleted:    src/symbolize/noop.rs
	deleted:    src/types.rs
	deleted:    src/windows.rs
	deleted:    tests/accuracy/auxiliary.rs
	deleted:    tests/accuracy/main.rs
	deleted:    tests/concurrent-panics.rs
	deleted:    tests/long_fn_name.rs
	deleted:    tests/skip_inner_frames.rs
	deleted:    tests/smoke.rs

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	.gitattributes
	.github/ISSUE_TEMPLATE/
	.github/workflows/ci.yml
	.mailmap
	CODE_OF_CONDUCT.md
	CONTRIBUTING.md
	COPYRIGHT
	Cargo.lock
	RELEASES.md
	compiler/
	config.toml.example
	configure
	library/
	rustfmt.toml
	src/README.md
	src/bootstrap/
	src/build_helper/
	src/ci/
	src/doc/
	src/etc/
	src/librustdoc/
	src/stage0.txt
	src/test/
	src/tools/
	src/version
	triagebot.toml
	x.py
