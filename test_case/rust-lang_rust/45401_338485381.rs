
[01:04:05] Dist RLS stage2 (x86_64-pc-windows-msvc)
[01:04:05] Building stage2 tool rls (x86_64-pc-windows-msvc)
...
[01:04:57] error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
[01:04:57]   --> src\tools\rustfmt\src\utils.rs:40:9
[01:04:57]    |
[01:04:57] 40 |         Visibility::Crate(_) => Cow::from("pub(crate) "),
[01:04:57]    |         ^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 1
[01:04:57] 
[01:05:00] error: aborting due to previous error
[01:05:00] 
[01:05:02] error: Could not compile `rustfmt-nightly`.
[01:05:02] warning: build failed, waiting for other jobs to finish...
[01:05:04] error: build failed
