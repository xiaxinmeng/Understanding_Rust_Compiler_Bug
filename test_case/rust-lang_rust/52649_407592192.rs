plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/cc/1a/bb0bc699b37a766736b0c07a7344b1b985deb16870e9d14c75110ae74256/awscli-1.15.64-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 19.5MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
Testing syntax_ext stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:00]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
0m missing `position`
[01:10:02] ...
[01:10:02] 673 | /             assert_pns_eq_sub!("%c",
[01:10:02] 674 | |                 { None, "", None, None, None, "c", });
[01:10:02]     | |______________________________________________________- in this macro invocation
[01:10:02] 
[01:10:02] error[E0063]: missing field `position` in initializer of `format_foreign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     |                                         ^ missing `position`
[01:10:02] ...
[01:10:02] 675 | /             assert_pns_eq_sub!("%s",
[01:10:02] 676 | |                 { None, "", None, None, None, "s", });
[01:10:02]     | |______________________________________________________- in this macro invocation
[01:10:02] 
[01:10:02] error[E0063]: missing field `position` in initializer of `format_foreign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     |                                         ^ missing `position`
[01:10:02] ...
[01:10:02] 677 | /             assert_pns_eq_sub!("%06d",
[01:10:02] 678 | |                 { None, "0", Some(N::Num(6)), None, None, "d", });
[01:10:02]     | |__________________________________________________________________- in this macro invocation
[01:10:02] 
[01:10:02] error[E0063]: missing field `position` in initializer of `format_foreign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     reign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     |                                         ^ missing `position`
[01:10:02] ...
[01:10:02] 691 | /             assert_pns_eq_sub!("%.6i",
[01:10:02] 692 | |                 { None, "", None, Some(N::Num(6)), None, "i", });
[01:10:02]     | |_________________________________________________________________- in this macro invocation
[01:10:02] 
[01:10:02] error[E0063]: missing field `position` in initializer of `format_foreign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     |                                         ^ missing `position`
[01:10:02] ...
[01:10:02] 693 | /             assert_pns_eq_sub!("%+i",
[01:10:02] 694 | |                 { None, "+", None, None, None, "i", });
[01:10:02]     | |_______________________________________________________- in this macro invocation
[01:10:02] 
[01:10:02] error[E0063]: missing field `position` in initializer of `format_foreign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     |                                         ^ missing `position`
[01:10:02] ...
[01:10:02] 695 | /             assert_pns_eq_sub!("%08X",
[01:10:02] 696 | |                 { None, "0", Some(N::Num(8)), None, None, "X", });
[01:10:02]     | |__________________________________________________________________- in this macro invocation
[01:10:02] 
[01:10:02] error[E0063]: missing field `position` in initializer of `format_foreign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     |                                         ^ missing `position`
[01:10:02] ...
[01:10:02] 697 | /             assert_pns_eq_sub!("%lu",
[01:10:02] 698 | |                 { None, "", None, None, Some("l"), "u", });
[01:10:02]     | |___________________________________________________________- in this macro invocation
[01:10:02] 
[01:10:02] error[E0063]: missing field `position` in initializer of `format_foreign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     |                                         ^ missing `position`
[01:10:02] ...
[01:10:02] 699 | /             assert_pns_eq_sub!("%Iu",
[01:10:02] 700 | |                 { None, "", None, None, Some("I"), "u", });
[01:10:02]     | |___________________________________________________________- in this macro invocation
[01:10:02] 
[01:10:02] error[E0063]: missing field `position` in initializer of `format_foreign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     |                                         ^ missing `position`
[01:10:02] ...
[01:10:02] 701 | /             assert_pns_eq_sub!("%I32u",
[01:10:02] 702 | |                 { None, "", None, None, Some("I32"), "u", });
[01:10:02]     | |_____________________________________________________________- in this macro invocation
[01:10:02] 
[01:10:02] error[E0063]: missing field `position` in initializer of `format_foreign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     |                                         ^ missing `position`
[01:10:02] ...
[01:10:02] 703 | /             assert_pns_eq_sub!("%I64u",
[01:10:02] 704 | |                 { None, "", None, None, Some("I64"), "u", });
[01:10:02]     | |_____________________________________________________________- in this macro invocation
[01:10:02] 
[01:10:02] error[E0063]: missing field `position` in initializer of `format_foreign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02] 656 |                        
[01:10:02]     |
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     |                                         ^ missing `position`
[01:10:02] ...
[01:10:02] 711 | /             assert_pns_eq_sub!("%1$d",
[01:10:02] 712 | |                 { Some(1), "", None, None, None, "d", });
[01:10:02]     | |_________________________________________________________- in this macro invocation
[01:10:02] 
[01:10:02] error[E0063]: missing field `position` in initializer of `format_foreign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     |                                         ^ missing `position`
[01:10:02] ...
[01:10:02] 713 | /             assert_pns_eq_sub!("%2$.*3$d",
[01:10:02] 714 | |                 { Some(2), "", None, Some(N::Arg(3)), None, "d", });
[01:10:02]     | |____________________________________________________________________- in this macro invocation
[01:10:02] 
[01:10:02] error[E0063]: missing field `position` in initializer of `format_foreign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     |                                         ^ missing `position`
[01:10:02] ...
[01:10:02] 715 | /             assert_pns_eq_sub!("%1$*2$.*3$d",
[01:10:02] 716 | |                 { Some(1), "", Some(N::Arg(2)), Some(N::Arg(3)), None, "d", });
[01:10:02]     | |_______________________________________________________________________________- in this macro invocation
[01:10:02] 
[01:10:02] error[E0063]: missing field `position` in initializer of `format_foreign::printf::Format<'_>`
[01:10:02]    --> libsyntax_ext/format_foreign.rs:656:39
[01:10:02]     |
[01:10:02] 656 |                               S::Format(F {
[01:10:02]     |                                         ^ missing `position`
[01:10:02] ...
[01:10:02] 717 | /             assert_pns_eq_sub!("%-8ld",
[01:10:02] 718 | |                 { None, "-", Some(N::Num(8)), None, Some("l"), "d", });
[01:10:02]     | |_______________________________________________________________________- in this macro invocation
[01:10:02] error: aborting due to 24 previous errors
[01:10:02] 
[01:10:02] For more information about this error, try `rustc --explain E0063`.
[01:10:02] error: Could not compile `syntax_ext`.
[01:10:02] error: Could not compile `syntax_ext`.
[01:10:02] 
[01:10:02] To learn more, run the command again with --verbose.
[01:10:02] 
[01:10:02] 
[01:10:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax_ext" "--" "--quiet"
[01:10:02] 
[01:10:02] 
[01:10:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:02] Build completed unsuccessfully in 0:28:27
[01:10:02] Build completed unsuccessfully in 0:28:27
[01:10:02] make: *** [check] Error 1
[01:10:02] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:070795d4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
