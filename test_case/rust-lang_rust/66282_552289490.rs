plain
2019-11-11T04:30:25.1341610Z [RUSTC-TIMING] rustc_traits test:false 65.160
2019-11-11T04:30:25.1342123Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-11-11T04:31:08.6176939Z [RUSTC-TIMING] rustc_passes test:false 43.476
2019-11-11T04:31:08.6231708Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-11-11T04:31:23.8394662Z error[E0382]: use of moved value: `vf_s0`
2019-11-11T04:31:23.8395143Z   --> src/librustc_mir/transform/simplify_try.rs:54:35
2019-11-11T04:31:23.8395434Z    |
2019-11-11T04:31:23.8396607Z 45 |             let (local_tmp_s0, local_1, vf_s0) = match match_get_variant_field(s0) {
2019-11-11T04:31:23.8397180Z    |                                         ----- move occurs because `vf_s0` has type `transform::simplify_try::VarField<'_>`, which does not implement the `Copy` trait
2019-11-11T04:31:23.8397656Z ...
2019-11-11T04:31:23.8398144Z 53 |             if (local_tmp_s0, vf_s0) != (local_tmp_s1, vf_s1)
2019-11-11T04:31:23.8398573Z    |                               ----- value moved here
2019-11-11T04:31:23.8399031Z 54 |                 || Some((local_0, vf_s0.var_idx)) != match_set_discr(s2)
2019-11-11T04:31:23.8399508Z    |                                   ^^^^^^^^^^^^^ value used here after move
2019-11-11T04:31:27.2401323Z error: aborting due to previous error
2019-11-11T04:31:27.2405768Z 
2019-11-11T04:31:27.2416558Z For more information about this error, try `rustc --explain E0382`.
2019-11-11T04:31:27.3742025Z [RUSTC-TIMING] rustc_mir test:false 18.745
---
2019-11-11T04:32:48.4672990Z   local time: Mon Nov 11 04:32:48 UTC 2019
2019-11-11T04:32:48.7846213Z   network time: Mon, 11 Nov 2019 04:32:48 GMT
2019-11-11T04:32:48.7847517Z == end clock drift check ==
2019-11-11T04:32:51.6983858Z 
2019-11-11T04:32:51.7095887Z ##[error]Bash exited with code '1'.
2019-11-11T04:32:51.7135441Z ##[section]Starting: Checkout
2019-11-11T04:32:51.7137620Z ==============================================================================
2019-11-11T04:32:51.7137736Z Task         : Get sources
2019-11-11T04:32:51.7137852Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
