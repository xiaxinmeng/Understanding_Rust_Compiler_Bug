plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error: use of associated function `path::Path::metadata` that will be deprecated in future version 1.51.0: the `std::fs::metadata` function is now preferred
   --> library/std/src/fs/tests.rs:402:41
    |
402 |     let stat_res_meth = check!(filename.metadata());
    |                                         ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::metadata`
    |
    = note: `-D deprecated-in-future` implied by `-D warnings`

error: use of associated function `path::Path::metadata` that will be deprecated in future version 1.51.0: the `std::fs::metadata` function is now preferred
   --> library/std/src/fs/tests.rs:414:41
    |
414 |     let stat_res_meth = check!(filename.metadata());
    |                                         ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::metadata`

error: use of associated function `path::Path::is_file` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace files at any time
   --> library/std/src/fs/tests.rs:424:18
    |
424 |     assert!(!dir.is_file());
    |                  ^^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::File::open` or `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:433:18
    |
433 |     assert!(file.exists());
    |                  ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:435:19
    |
435 |     assert!(!file.exists());
    |                   ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:442:18
    |
442 |     assert!(!dir.exists());
    |                  ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:444:17
    |
444 |     assert!(dir.exists());
    |                 ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::is_dir` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace directories at any time
   --> library/std/src/fs/tests.rs:445:17
    |
445 |     assert!(dir.is_dir());
    |                 ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:447:18
    |
447 |     assert!(!dir.exists());
    |                  ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::is_dir` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace directories at any time
   --> library/std/src/fs/tests.rs:502:17
    |
502 |     assert!(dir.is_dir())
    |                 ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``

error: use of associated function `path::Path::is_dir` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace directories at any time
   --> library/std/src/fs/tests.rs:570:17
    |
570 |     assert!(!d1.is_dir());
    |                 ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:571:20
    |
571 |     assert!(canary.exists());
    |                    ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::is_dir` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace directories at any time
   --> library/std/src/fs/tests.rs:586:19
    |
586 |     assert!(!link.is_dir());
    |                   ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:587:20
    |
587 |     assert!(canary.exists());
    |                    ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::is_dir` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace directories at any time
   --> library/std/src/fs/tests.rs:611:28
    |
611 |     assert!(Path::new(".").is_dir());
    |                            ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``

error: use of associated function `path::Path::is_dir` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace directories at any time
   --> library/std/src/fs/tests.rs:612:46
    |
612 |     assert!(!Path::new("test/stdtest/fs.rs").is_dir());
    |                                              ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``

error: use of associated function `path::Path::is_dir` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace directories at any time
   --> library/std/src/fs/tests.rs:619:21
    |
619 |     assert!(dirpath.is_dir());
    |                     ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``

error: use of associated function `path::Path::is_dir` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace directories at any time
   --> library/std/src/fs/tests.rs:624:23
    |
624 |     assert!(!filepath.is_dir());
    |                       ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:625:22
    |
625 |     assert!(filepath.exists());
    |                      ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:630:28
    |
630 |     assert!(Path::new(".").exists());
    |                            ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:631:55
    |
631 |     assert!(!Path::new("test/nonexistent-bogus-path").exists());
    |                                                       ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:637:21
    |
637 |     assert!(unicode.exists());
    |                     ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:638:57
    |
638 |     assert!(!Path::new("test/unicode-bogus-path-각丁ー再见").exists());
    |                                                              ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:649:27
    |
649 |             assert!(!from.exists());
    |                           ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:650:25
    |
650 |             assert!(!to.exists());
    |                         ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:662:19
    |
662 |     assert!(!from.exists());
    |                   ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::metadata` that will be deprecated in future version 1.51.0: the `std::fs::metadata` function is now preferred
   --> library/std/src/fs/tests.rs:680:29
    |
680 |     assert_eq!(check!(input.metadata()).permissions(), check!(out.metadata()).permissions());
    |                             ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::metadata`

error: use of associated function `path::Path::metadata` that will be deprecated in future version 1.51.0: the `std::fs::metadata` function is now preferred
   --> library/std/src/fs/tests.rs:680:67
    |
680 |     assert_eq!(check!(input.metadata()).permissions(), check!(out.metadata()).permissions());
    |                                                                   ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::metadata`

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
   --> library/std/src/fs/tests.rs:719:18
    |
719 |     assert!(!out.exists());
    |                  ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::metadata` that will be deprecated in future version 1.51.0: the `std::fs::metadata` function is now preferred
   --> library/std/src/fs/tests.rs:733:24
    |
733 |     assert!(check!(out.metadata()).permissions().readonly());
    |                        ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::metadata`

error: use of associated function `path::Path::metadata` that will be deprecated in future version 1.51.0: the `std::fs::metadata` function is now preferred
   --> library/std/src/fs/tests.rs:744:43
    |
744 |     assert_eq!(check!(tmp.join("out.txt").metadata()).len(), 0);
    |                                           ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::metadata`

error: use of associated function `path::Path::metadata` that will be deprecated in future version 1.51.0: the `std::fs::metadata` function is now preferred
   --> library/std/src/fs/tests.rs:759:32
    |
759 |     assert_eq!(check!(out_path.metadata()).len(), copied_len);
    |                                ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::metadata`

error: use of associated function `path::Path::symlink_metadata` that will be deprecated in future version 1.51.0: the `std::fs::symlink_metadata` function is now preferred
   --> library/std/src/fs/tests.rs:779:37
    |
779 |     assert!(check!(out_path_symlink.symlink_metadata()).file_type().is_symlink());
    |                                     ^^^^^^^^^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::symlink_metadata`

error: use of associated function `path::Path::symlink_metadata` that will be deprecated in future version 1.51.0: the `std::fs::symlink_metadata` function is now preferred
   --> library/std/src/fs/tests.rs:796:24
    |
796 |     assert!(check!(out.symlink_metadata()).file_type().is_symlink());
    |                        ^^^^^^^^^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::symlink_metadata`

error: use of associated function `path::Path::metadata` that will be deprecated in future version 1.51.0: the `std::fs::metadata` function is now preferred
   --> library/std/src/fs/tests.rs:863:63
    |
863 |     assert_eq!(check!(fs::metadata(&out)).len(), check!(input.metadata()).len());
    |                                                               ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::metadata`

error: use of associated function `path::Path::symlink_metadata` that will be deprecated in future version 1.51.0: the `std::fs::symlink_metadata` function is now preferred
    --> library/std/src/fs/tests.rs:1199:18
     |
1199 |     assert!(link.symlink_metadata().unwrap().file_type().is_symlink());
     |                  ^^^^^^^^^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::symlink_metadata`

error: use of associated function `path::Path::read_dir` that will be deprecated in future version 1.51.0: the `std::fs::read_dir` function is now preferred
    --> library/std/src/fs/tests.rs:1246:31
     |
1246 |     for file in tmpdir.path().read_dir().unwrap().map(|f| f.unwrap()) {
     |                               ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::read_dir`

error: use of associated function `path::Path::read_dir` that will be deprecated in future version 1.51.0: the `std::fs::read_dir` function is now preferred
    --> library/std/src/fs/tests.rs:1266:38
     |
1266 |     let mut read_dir = tmpdir.path().read_dir().unwrap();
     |                                      ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::read_dir`

error: use of associated function `path::Path::is_dir` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace directories at any time
    --> library/std/src/fs/tests.rs:1296:22
     |
1296 |     assert!(junction.is_dir());
     |                      ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
    --> library/std/src/fs/tests.rs:1297:15
     |
1297 |     assert!(b.exists());
     |               ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of associated function `path::Path::is_dir` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace directories at any time
    --> library/std/src/fs/tests.rs:1304:18
     |
1304 |     assert!(link.is_dir());
     |                  ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``

error: use of associated function `path::Path::exists` that will be deprecated in future version 1.51.0: other processes may remove or rename files at any time
    --> library/std/src/fs/tests.rs:1305:15
     |
1305 |     assert!(d.exists());
     |               ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``
error: aborting due to 42 previous errors

error: could not compile `std`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "core" "-p" "std" "-p" "alloc" "-p" "unwind" "-p" "panic_abort" "-p" "panic_unwind" "-p" "term" "-p" "proc_macro" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:40
