
library/std/src/path.rs:2279:    /// `&self.parent().unwrap().parent().unwrap()` and so on. If the [`parent`] method returns
src/bootstrap/config.rs:819:        config.src = manifest_dir.parent().unwrap().parent().unwrap().to_owned();
src/bootstrap/test.rs:1729:                .arg(builder.cc(target).parent().unwrap().parent().unwrap());
src/tools/compiletest/src/runtest.rs:2933:        let src_root = self.config.src_base.parent().unwrap().parent().unwrap();
src/tools/compiletest/src/runtest.rs:3550:            Some(self.config.src_base.parent().unwrap().parent().unwrap().into()),
src/tools/compiletest/src/runtest.rs:3573:        let parent_build_dir = test_build_dir.parent().unwrap().parent().unwrap().parent().unwrap();
src/tools/rust-analyzer/crates/rust-analyzer/src/cli/load_cargo.rs:156:        let path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
src/tools/rust-analyzer/crates/sourcegen/src/lib.rs:199:    let res = PathBuf::from(dir).parent().unwrap().parent().unwrap().to_owned();
src/tools/rust-analyzer/crates/test-utils/src/lib.rs:407:    PathBuf::from(dir).parent().unwrap().parent().unwrap().to_owned()
