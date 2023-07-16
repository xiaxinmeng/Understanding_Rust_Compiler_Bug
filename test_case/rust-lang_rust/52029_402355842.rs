
Mon Jul  2 19:47:22 CEST 2018: info: building: 1.28.0-beta.6 (915ac6602 2018-06-30)
Mon Jul  2 19:47:22 CEST 2018: info: required stage0:
	date: 2018-06-21
	rustc: 1.27.0
	cargo: 0.28.0
Mon Jul  2 19:47:22 CEST 2018: info: rustc -vV
	rustc 1.27.0
	binary: rustc
	commit-hash: unknown
	commit-date: unknown
	host: x86_64-unknown-openbsd
	release: 1.27.0
	LLVM version: 6.0
Mon Jul  2 19:47:22 CEST 2018: info: cargo -vV
	cargo 1.27.0
	release: 1.27.0
Mon Jul  2 19:47:22 CEST 2018: starting rustbuild dist --jobs=4
running: /usr/local/bin/cargo build --manifest-path /data/semarie/build-rust/build_dir/rustc-beta-src/src/bootstrap/Cargo.toml --frozen
    Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
error: failed to load source for a dependency on `rustfmt-nightly`

Caused by:
  Unable to update https://github.com/rust-lang-nursery/rustfmt?rev=f3906267#f3906267

Caused by:
  failed to fetch into /data/semarie/build-rust/install_dir/crates/git/db/rustfmt-5390e0ead582d971

Caused by:
  attempting to update a git repository, but --frozen was specified
Traceback (most recent call last):
  File "/data/semarie/build-rust/build_dir/rustc-beta-src/x.py", line 20, in <module>
    bootstrap.main()
  File "/data/semarie/build-rust/build_dir/rustc-beta-src/src/bootstrap/bootstrap.py", line 827, in main
    bootstrap(help_triggered)
  File "/data/semarie/build-rust/build_dir/rustc-beta-src/src/bootstrap/bootstrap.py", line 806, in bootstrap
    build.build_bootstrap()
  File "/data/semarie/build-rust/build_dir/rustc-beta-src/src/bootstrap/bootstrap.py", line 633, in build_bootstrap
    run(args, env=env, verbose=self.verbose)
  File "/data/semarie/build-rust/build_dir/rustc-beta-src/src/bootstrap/bootstrap.py", line 148, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /usr/local/bin/cargo build --manifest-path /data/semarie/build-rust/build_dir/rustc-beta-src/src/bootstrap/Cargo.toml --frozen
