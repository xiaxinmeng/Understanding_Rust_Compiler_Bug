
---- [rustdoc] rustdoc/all.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
[src/librustdoc/html/format.rs:490] did = DefId(0:16 ~ foo[cf51]::private_module::ReexportedStruct)
thread 'rustc' panicked at 'DefId(0:16 ~ foo[cf51]::private_module::ReexportedStruct)', src/librustdoc/html/format.rs:491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-apple-darwin

query stack during panic:
end of query stack

------------------------------------------


---- [rustdoc] rustdoc/inline_cross/renamed-via-module.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
[src/librustdoc/html/format.rs:490] did = DefId(0:5 ~ foo[cf51]::iter::range::StepBy)
thread 'rustc' panicked at 'DefId(0:5 ~ foo[cf51]::iter::range::StepBy)', src/librustdoc/html/format.rs:491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-apple-darwin

query stack during panic:
end of query stack

------------------------------------------


---- [rustdoc] rustdoc/inline_local/glob-private-no-defaults.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
21: @!has check failed
	`PATTERN` did not match
	// @!has - "mod2"
22: @has check failed
	`PATTERN` did not match
	// @has - "Mod2Public"
26: @has check failed
	File does not exist 'foo/struct.Mod2Public.html'
	// @has foo/struct.Mod2Public.html

Encountered 3 errors

------------------------------------------

info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/inline_local/glob-private-no-defaults.rs' panicked at 'assertion failed: status.success() || status.code() == Some(1)', src/tools/compiletest/src/runtest.rs:2484:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [rustdoc] rustdoc/inline_local/issue-28537.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
[src/librustdoc/html/format.rs:490] did = DefId(0:11 ~ issue_28537[2196]::bar::bar::Bar)
thread 'rustc' panicked at 'DefId(0:11 ~ issue_28537[2196]::bar::bar::Bar)', src/librustdoc/html/format.rs:491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-apple-darwin

query stack during panic:
end of query stack

------------------------------------------


---- [rustdoc] rustdoc/inline_local/issue-32343.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
[src/librustdoc/html/format.rs:490] did = DefId(0:12 ~ issue_32343[11e0]::foo::Bar)
thread 'rustc' panicked at 'DefId(0:12 ~ issue_32343[11e0]::foo::Bar)', src/librustdoc/html/format.rs:491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-apple-darwin

query stack during panic:
end of query stack

------------------------------------------


---- [rustdoc] rustdoc/inline_local/glob-private.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
19: @!has check failed
	`PATTERN` did not match
	// @!has - "mod2"
20: @has check failed
	`PATTERN` did not match
	// @has - "Mod2Public"
24: @has check failed
	File does not exist 'foo/struct.Mod2Public.html'
	// @has foo/struct.Mod2Public.html
34: @has-dir check failed
	Directory does not exist 'foo/mod1/mod2'
	// @has-dir foo/mod1/mod2
36: @has check failed
	File does not exist 'foo/mod1/mod2/struct.Mod2Public.html'
	// @has foo/mod1/mod2/struct.Mod2Public.html

Encountered 5 errors

------------------------------------------

info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/inline_local/glob-private.rs' panicked at 'assertion failed: status.success() || status.code() == Some(1)', src/tools/compiletest/src/runtest.rs:2484:21

---- [rustdoc] rustdoc/inline_local/trait-vis.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
[src/librustdoc/html/format.rs:490] did = DefId(0:8 ~ trait_vis[70a0]::asdf::SomeStruct)
thread 'rustc' panicked at 'DefId(0:8 ~ trait_vis[70a0]::asdf::SomeStruct)', src/librustdoc/html/format.rs:491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-apple-darwin

query stack during panic:
end of query stack

------------------------------------------


---- [rustdoc] rustdoc/intra-doc/cross-crate/hidden.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
warning: lint `broken_intra_doc_links` has been renamed to `rustdoc::broken_intra_doc_links`
 --> [FULL PATH REDACTED]
  |
2 | #![deny(broken_intra_doc_links)]
  |         ^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `rustdoc::broken_intra_doc_links`
  |
  = note: `#[warn(renamed_and_removed_lints)]` on by default

[src/librustdoc/html/format.rs:490] did = DefId(0:7 ~ hidden_dep[505c]::future::ready::Ready)
thread 'rustc' panicked at 'DefId(0:7 ~ hidden_dep[505c]::future::ready::Ready)', src/librustdoc/html/format.rs:491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-apple-darwin

query stack during panic:
end of query stack
warning: 1 warning emitted


------------------------------------------


---- [rustdoc] rustdoc/issue-34473.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
[src/librustdoc/html/format.rs:490] did = DefId(0:4 ~ foo[cf51]::second::SomeTypeWithLongName)
thread 'rustc' panicked at 'DefId(0:4 ~ foo[cf51]::second::SomeTypeWithLongName)', src/librustdoc/html/format.rs:491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-apple-darwin

query stack during panic:
end of query stack

------------------------------------------


---- [rustdoc] rustdoc/namespaces.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
[src/librustdoc/html/format.rs:490] did = DefId(0:5 ~ namespaces[e9ef]::inner::sync::SomeStruct)
thread 'rustc' panicked at 'DefId(0:5 ~ namespaces[e9ef]::inner::sync::SomeStruct)', src/librustdoc/html/format.rs:491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-apple-darwin

query stack during panic:
end of query stack

------------------------------------------


---- [rustdoc] rustdoc/redirect-map.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
[src/librustdoc/html/format.rs:490] did = DefId(0:10 ~ foo[cf51]::private::Quz)
thread 'rustc' panicked at 'DefId(0:10 ~ foo[cf51]::private::Quz)', src/librustdoc/html/format.rs:491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

error: Unrecognized option: 'generate-redirect-map'


------------------------------------------


---- [rustdoc] rustdoc/redirect-rename.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
[src/librustdoc/html/format.rs:490] did = DefId(0:11 ~ foo[cf51]::hidden::bar::Thing)
thread 'rustc' panicked at 'DefId(0:11 ~ foo[cf51]::hidden::bar::Thing)', src/librustdoc/html/format.rs:491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-apple-darwin

query stack during panic:
end of query stack

------------------------------------------


---- [rustdoc] rustdoc/redirect.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
[src/librustdoc/html/format.rs:490] did = DefId(0:10 ~ reexp_stripped[c7ca]::private::Quz)
thread 'rustc' panicked at 'DefId(0:10 ~ reexp_stripped[c7ca]::private::Quz)', src/librustdoc/html/format.rs:491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-apple-darwin

query stack during panic:
end of query stack

------------------------------------------


---- [rustdoc] rustdoc/search-index.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
[src/librustdoc/html/format.rs:490] did = DefId(0:10 ~ rustdoc_test[88de]::private::Foo)
thread 'rustc' panicked at 'DefId(0:10 ~ rustdoc_test[88de]::private::Foo)', src/librustdoc/html/format.rs:491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-apple-darwin

query stack during panic:
end of query stack

------------------------------------------


---- [rustdoc] rustdoc/synthetic_auto/complex.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: [REDACTED]
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
[src/librustdoc/html/format.rs:490] did = DefId(0:7 ~ complex[87cf]::foo::Inner)
thread 'rustc' panicked at 'DefId(0:7 ~ complex[87cf]::foo::Inner)', src/librustdoc/html/format.rs:491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-apple-darwin

query stack during panic:
end of query stack

------------------------------------------
