
Dec 11 00:12:01 ip-10-202-181-185 release-nightly: Building stage0 tool build-manifest (x86_64-unknown-linux-gnu)
Dec 11 00:12:02 ip-10-202-181-185 release-nightly:    Compiling proc-macro2 v0.4.24
Dec 11 00:12:02 ip-10-202-181-185 release-nightly:    Compiling unicode-xid v0.1.0
Dec 11 00:12:02 ip-10-202-181-185 release-nightly:    Compiling serde v1.0.75
Dec 11 00:12:22 ip-10-202-181-185 release-nightly:    Compiling quote v0.6.8
Dec 11 00:12:33 ip-10-202-181-185 release-nightly:    Compiling toml v0.4.6
Dec 11 00:12:33 ip-10-202-181-185 release-nightly:    Compiling syn v0.15.21
Dec 11 00:13:12 ip-10-202-181-185 release-nightly:    Compiling serde_derive v1.0.80
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:    Compiling build-manifest v0.1.0 (/tmp/nightly/rust/src/tools/build-manifest)
Dec 11 00:14:04 ip-10-202-181-185 release-nightly: error[E0433]: failed to resolve. Use of undeclared type or module `fs`
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:    --> src/tools/build-manifest/src/main.rs:609:12
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:     |
Dec 11 00:14:04 ip-10-202-181-185 release-nightly: 609 |         t!(fs::write(&sha256, &sha.stdout));
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:     |            ^^ Use of undeclared type or module `fs`
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:
Dec 11 00:14:04 ip-10-202-181-185 release-nightly: error[E0433]: failed to resolve. Use of undeclared type or module `fs`
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:    --> src/tools/build-manifest/src/main.rs:646:12
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:     |
Dec 11 00:14:04 ip-10-202-181-185 release-nightly: 646 |         t!(fs::write(&dst, contents));
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:     |            ^^ Use of undeclared type or module `fs`
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:
Dec 11 00:14:04 ip-10-202-181-185 release-nightly: error: unused import: `std::fs::File`
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:   --> src/tools/build-manifest/src/main.rs:17:5
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:    |
Dec 11 00:14:04 ip-10-202-181-185 release-nightly: 17 | use std::fs::File;
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:    |     ^^^^^^^^^^^^^
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:    |
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:    = note: `-D unused-imports` implied by `-D warnings`
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:
Dec 11 00:14:04 ip-10-202-181-185 release-nightly: error: aborting due to 3 previous errors
Dec 11 00:14:04 ip-10-202-181-185 release-nightly:
Dec 11 00:14:04 ip-10-202-181-185 release-nightly: For more information about this error, try `rustc --explain E0433`.
Dec 11 00:14:04 ip-10-202-181-185 release-nightly: error: Could not compile `build-manifest`.

