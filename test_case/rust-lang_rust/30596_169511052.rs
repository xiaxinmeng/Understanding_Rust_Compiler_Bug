
rustc: x86_64-unknown-dragonfly/stage2/lib/rustlib/x86_64-unknown-dragonfly/lib/libsyntax
../src/libsyntax/util/interner.rs:95:34: 95:44 error: unknown `allow` attribute: `unused_qualifications`, #[deny(unknown_lints)] on by default
../src/libsyntax/util/interner.rs:95 #[derive(Clone, PartialEq, Hash, PartialOrd)]
                                                                      ^~~~~~~~~~
../src/libsyntax/util/interner.rs:95:34: 95:44 note: in this expansion of #[derive_PartialOrd] (defined in ../src/libsyntax/util/interner.rs)
../src/libsyntax/util/interner.rs:95:28: 95:32 error: unknown `allow` attribute: `unused_qualifications`, #[deny(unknown_lints)] on by default
../src/libsyntax/util/interner.rs:95 #[derive(Clone, PartialEq, Hash, PartialOrd)]
                                                                ^~~~
../src/libsyntax/util/interner.rs:95:28: 95:32 note: in this expansion of #[derive_Hash] (defined in ../src/libsyntax/util/interner.rs)
../src/libsyntax/util/interner.rs:95:17: 95:26 error: unknown `allow` attribute: `unused_qualifications`, #[deny(unknown_lints)] on by default
../src/libsyntax/util/interner.rs:95 #[derive(Clone, PartialEq, Hash, PartialOrd)]
                                                     ^~~~~~~~~
../src/libsyntax/util/interner.rs:95:17: 95:26 note: in this expansion of #[derive_PartialEq] (defined in ../src/libsyntax/util/interner.rs)
../src/libsyntax/util/interner.rs:95:10: 95:15 error: unknown `allow` attribute: `unused_qualifications`, #[deny(unknown_lints)] on by default
../src/libsyntax/util/interner.rs:95 #[derive(Clone, PartialEq, Hash, PartialOrd)]
                                              ^~~~~
../src/libsyntax/util/interner.rs:95:10: 95:15 note: in this expansion of #[derive_Clone] (defined in ../src/libsyntax/util/interner.rs)
../src/libsyntax/util/parser.rs:16:28: 16:30 error: unknown `allow` attribute: `unused_qualifications`, #[deny(unknown_lints)] on by default
../src/libsyntax/util/parser.rs:16 #[derive(Debug, PartialEq, Eq)]
                                                              ^~
../src/libsyntax/util/parser.rs:16:28: 16:30 note: in this expansion of #[derive_Eq] (defined in ../src/libsyntax/util/parser.rs)
../src/libsyntax/util/parser.rs:16:17: 16:26 error: unknown `allow` attribute: `unused_qualifications`, #[deny(unknown_lints)] on by default
../src/libsyntax/util/parser.rs:16 #[derive(Debug, PartialEq, Eq)]
                                                   ^~~~~~~~~
../src/libsyntax/util/parser.rs:16:17: 16:26 note: in this expansion of #[derive_PartialEq] (defined in ../src/libsyntax/util/parser.rs)
../src/libsyntax/util/parser.rs:16:10: 16:15 error: unknown `allow` attribute: `unused_qualifications`, #[deny(unknown_lints)] on by default
../src/libsyntax/util/parser.rs:16 #[derive(Debug, PartialEq, Eq)]
                                            ^~~~~
../src/libsyntax/util/parser.rs:16:10: 16:15 note: in this expansion of #[derive_Debug] (defined in ../src/libsyntax/util/parser.rs)
../src/libsyntax/util/parser.rs:66:28: 66:30 error: unknown `allow` attribute: `unused_qualifications`, #[deny(unknown_lints)] on by default
../src/libsyntax/util/parser.rs:66 #[derive(Debug, PartialEq, Eq)]
                                                              ^~
../src/libsyntax/util/parser.rs:66:28: 66:30 note: in this expansion of #[derive_Eq] (defined in ../src/libsyntax/util/parser.rs)
../src/libsyntax/util/parser.rs:66:17: 66:26 error: unknown `allow` attribute: `unused_qualifications`, #[deny(unknown_lints)] on by default
../src/libsyntax/util/parser.rs:66 #[derive(Debug, PartialEq, Eq)]
                                                   ^~~~~~~~~
../src/libsyntax/util/parser.rs:66:17: 66:26 note: in this expansion of #[derive_PartialEq] (defined in ../src/libsyntax/util/parser.rs)
../src/libsyntax/util/parser.rs:66:10: 66:15 error: unknown `allow` attribute: `unused_qualifications`, #[deny(unknown_lints)] on by default
../src/libsyntax/util/parser.rs:66 #[derive(Debug, PartialEq, Eq)] 
                                            ^~~~~
../src/libsyntax/util/parser.rs:66:10: 66:15 note: in this expansion of #[derive_Debug] (defined in ../src/libsyntax/util/parser.rs)
../src/libsyntax/diagnostics/registry.rs:13:10: 13:15 error: unknown `allow` attribute: `unused_qualifications`, #[deny(unknown_lints)] on by default
../src/libsyntax/diagnostics/registry.rs:13 #[derive(Clone)]
                                                     ^~~~~
../src/libsyntax/diagnostics/registry.rs:13:10: 13:15 note: in this expansion of #[derive_Clone] (defined in ../src/libsyntax/diagnostics/registry.rs)
../src/libsyntax/diagnostics/metadata.rs:31:37: 31:51 error: unknown `allow` attribute: `unused_qualifications`, #[deny(unknown_lints)] on by default
../src/libsyntax/diagnostics/metadata.rs:31 #[derive(PartialEq, RustcDecodable, RustcEncodable)]
                                                                                ^~~~~~~~~~~~~~
../src/libsyntax/diagnostics/metadata.rs:31:37: 31:51 note: in this expansion of #[derive_RustcEncodable] (defined in ../src/libsyntax/diagnostics/metadata.rs)
../src/libsyntax/diagnostics/metadata.rs:31:21: 31:35 error: unknown `allow` attribute: `unused_qualifications`, #[deny(unknown_lints)] on by default
../src/libsyntax/diagnostics/metadata.rs:31 #[derive(PartialEq, RustcDecodable, RustcEncodable)]
                                                                ^~~~~~~~~~~~~~
