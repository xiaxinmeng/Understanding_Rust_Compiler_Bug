text
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `None`,
 right: `Some([SubstitutionPart { span: /home/lukas/code/rust/tests/ui/parser/issues/issue-44406.rs:3:12: 3:24 (#4), snippet: " { " }, SubstitutionPart { span: /home/lukas/code/rust/tests/ui/parser/issues/issue-44406.rs:3:12: 3:24 (#4), snippet: " }" }])`: suggestion must not have overlapping parts', /home/lukas/code/rust/compiler/rustc_errors/src/diagnostic.rs:646:9
