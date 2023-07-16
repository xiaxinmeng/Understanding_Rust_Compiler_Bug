
[01:03:18] error[E0308]: mismatched types
[01:03:18]    --> /checkout/src/tools/rls/src/actions/mod.rs:183:55
[01:03:18]     |
[01:03:18] 183 |                         analysis.reload_from_analysis(new_analysis, &project_path, &cwd, false).unwrap();
[01:03:18]     |                                                       ^^^^^^^^^^^^ expected struct `rls_data::Analysis`, found struct `data::Analysis`
[01:03:18]     |
[01:03:18]     = note: expected type `rls_data::Analysis`
[01:03:18]                found type `data::Analysis`
[01:03:18] note: Perhaps two different versions of crate `rls_data` are being used?
[01:03:18]    --> /checkout/src/tools/rls/src/actions/mod.rs:183:55
[01:03:18]     |
[01:03:18] 183 |                         analysis.reload_from_analysis(new_analysis, &project_path, &cwd, false).unwrap();
[01:03:18]     |                                                       ^^^^^^^^^^^^
[01:03:18]
[01:03:18] error: aborting due to previous error
