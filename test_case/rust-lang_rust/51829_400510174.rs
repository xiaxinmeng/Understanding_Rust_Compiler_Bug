plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/ba/32/d6d254f6ccc2ed21f02d81f38709ff06feca9cbdb2e68ea90635fa483a73/awscli-1.15.46-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 7.6MB/s eta 0:00:01
    1% |▌                               | 20kB 1.7MB/s eta 0:00:01
    2% |▊                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
[01:33:11] travis_fold:start:test_stage1-syntax_ext
travis_time:start:test_stage1-syntax_ext
Testing syntax_ext stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:33:11]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:606:13
[01:33:13]     |
[01:33:13] 606 |             assert_eq!(pns("has no escapes"), None);
[01:33:13]     |
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:607:13
[01:33:13]     |
[01:33:13] 607 |             assert_eq!(pns("has no escapes, either %"), None);
[01:33:13]     |
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:638:13
[01:33:13]     |
[01:33:13] 638 | /             assert_pns_eq_sub!("%!",
[01:33:13] 639 | |                 { None, "", None, None, None, "!", });
[01:33:13]     | |______________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:642:13
[01:33:13]     |
[01:33:13]     |
[01:33:13] 642 | /             assert_pns_eq_sub!("%s",
[01:33:13] 643 | |                 { None, "", None, None, None, "s", });
[01:33:13]     | |______________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:644:13
[01:33:13]     |
[01:33:13] 644 | /             assert_pns_eq_sub!("%06d",
[01:33:13] 645 | |                 { None, "0", Some(N::Num(6)), None, None, "d", });
[01:33:13]     | |__________________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:646:13
[01:33:13]     |
[01:33:13] 646 | /             assert_pns_eq_sub!("%4.2f",
[01:33:13] 647 | |                 { None, "", Some(N::Num(4)), Some(N::Num(2)), None, "f", });
[01:33:13]     | |____________________________________________________________________________^ in this macro invocation
[01:33:13] he current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:650:13
[01:33:13]     |
[01:33:13] 650 | /             assert_pns_eq_sub!("%-10s",
[01:33:13] 651 | |                 { None, "-", Some(N::Num(10)), None, None, "s", });
[01:33:13]     | |___________________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:652:13
[01:33:13]     |
[01:33:13] 652 | /             assert_pns_eq_sub!("%*s",
[01:33:13] 653 | |                 { None, "", Some(N::Next), None, None, "s", });
[01:33:13]     | |_______________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:654:13
[01:33:13]     |
[01:33:13] 654 | /             assert_pns_eq_sub!("%-10.*s",
[01:33:13] 655 | |                 { None, "-", Some(N::Num(10)), Some(N::Next), None, "s", });
[01:33:13]     | |____________________________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:656:13
[01:33:13]     |
[01:33:13] 656 | /             assert_pns_eq_sub!("%-*.*s",
[01:33:13] 657 | |                 { None, "-", Some(N::Next), Some(N::Next), None, "s", });
[01:33:13]     | |_________________________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:658:13
[01:33:13]     |
[01:33:13] 658 | /             assert_pns_eq_sub!("%.6i",
[01:33:13] 659 | |                 { None, "", None, Some(N::Num(6)), None, "i", });
[01:33:13]     | |_________________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:660:13
[01:33:13]     |
[01:33:13] 660 | /             assert_pns_eq_sub!("%+i",
[01:33:13] 661 | |                 { None, "+", None, None, None, "i", });
[01:33:13]     | |_______________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:662:13
[01:33:13]     |
[01:33:13] 662 | /             assert_pns_eq_sub!("%08X",
[01:33:13] 663 | |                 { None, "0", Some(N::Num(8)), None, None, "X", });
[01:33:13]     | |__________________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:664:13
[01:33:13]     |
[01:33:13] 664 | /             assert_pns_eq_sub!("%lu",
[01:33:13] 665 | |                 { None, "", None, None, Some("l"), "u", });
[01:33:13]     | |___________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:666:13
[01:33:13]     |
[01:33:13] 666 | /             assert_pns_eq_sub!("%Iu",
[01:33:13] 667 | |                 { None, "", None, None, Some("I"), "u", });
[01:33:13]     | |___________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:668:13
[01:33:13]     |
[01:33:13] 668 | /             assert_pns_eq_sub!("%I32u",
[01:33:13] 669 | |                 { None, "", None, None, Some("I32"), "u", });
[01:33:13]     | |_____________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:670:13
[01:33:13]     |
[01:33:13] 670 | /             assert_pns_eq_sub!("%I64u",
[01:33:13] 671 | |                 { None, "", None, None, Some("I64"), "u", });
[01:33:13]     | |_____________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:672:13
[01:33:13]     |
[01:33:13] 672 | /             assert_pns_eq_sub!("%'d",
[01:33:13] 673 | |                 { None, "'", None, None, None, "d", });
[01:33:13]     | |_______________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:674:13
[01:33:13]     |
[01:33:13] 674 | /             assert_pns_eq_sub!("%10s",
[01:33:13] 675 | |                 { None, "", Some(N::Num(10)), None, None, "s", });
[01:33:13]     | |__________________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:676:13
[01:33:13]     |
[01:33:13] 676 | /             assert_pns_eq_sub!("%-10.10s",
[01:33:13] 677 | |                 { None, "-", Some(N::Num(10)), Some(N::Num(10)), None, "s", });
[01:33:13]     | |_______________________________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementati 682 | /             assert_pns_eq_sub!("%1$*2$.*3$d",
[01:33:13] 683 | |                 { Some(1), "", Some(N::Arg(2)), Some(N::Arg(3)), None, "d", });
[01:33:13]     | |_______________________________________________________________________________^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::printf::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:684:13
[01:33:13]     |
[01:33:13] 684 | /             assert_pns_eq_sub!("%-8ld",
[01:33:13] 685 | |                 { None, "-", Some(N::Num(8)), None, Some("lrrent crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:870:13
[01:33:13]     |
[01:33:13] 870 |             assert_eq!(pns("has no escapes, either $"), None);
[01:33:13]     |
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:871:13
[01:33:13]     |
[01:33:13] 871 |             assert_eq!(pns("*so* has a $$ escape"), Some((S::Escape, " escape")));
[01:33:13]     |
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:872:13
[01:33:13]     |
[01:33:13] 872 |             assert_eq!(pns("$$ leading escape"), Some((S::Escape, " leading escape")));
[01:33:13]     |
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:873:13
[01:33:13]     |
[01:33:13] 873 |             assert_eq!(pns("trailing escape $$"), Some((S::Escape, "")));
[01:33:13]     |
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:884:13
[01:33:13]     |
[01:33:13] 884 |             assert_pns_eq_sub!("$0", Ordinal(0));
[01:33:13]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:885:13
[01:33:13]     |
[01:33:13] 885 |             assert_pns_eq_sub!("$1", Ordinal(1));
[01:33:13]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:886:13
[01:33:13]     |
[01:33:13] 886 |             assert_pns_eq_sub!("$9", Ordinal(9));
[01:33:13]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation
[01:33:13]     |
[01:33:13]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13] 
[01:33:13] 
[01:33:13] error[E0369]: binary operation `==` cannot be applied to type `std::option::Option<(format_foreign::shell::Substitution<'_>, &str)>`
[01:33:13]    --> libsyntax_ext/format_foreign.rs:887:13
[01:33:13]     |
[01:33:13] 887 |      [0mnote: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:33:13] error: aborting due to 39 previous errors
[01:33:13] 
[01:33:13] For more information about this error, try `rustc --explain E0369`.
[01:33:13] error: Could not compile `syntax_ext`.
[01:33:13] error: Could not compile `syntax_ext`.
[01:33:13] 
[01:33:13] To learn more, run the command again with --verbose.
[01:33:13] 
[01:33:13] 
[01:33:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax_ext" "--" "--quiet"
[01:33:13] 
[01:33:13] 
[01:33:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:13] Build completed unsuccessfully in 0:44:42
[01:33:13] Build completed unsuccessfully in 0:44:42
[01:33:13] Makefile:58: recipe for target 'check' failed
[01:33:13] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15e7dc26
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
