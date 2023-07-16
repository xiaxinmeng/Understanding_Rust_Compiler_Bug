plain
test builder::tests::dist::dist_with_same_targets_and_hosts ... ok
test builder::tests::dist::dist_with_targets ... ok
test builder::tests::dist::dist_with_targets_and_hosts ... ok
test builder::tests::dist::doc_ci ... ok
fatal: ambiguous argument 'refs/remotes/origin/master..HEAD': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'
test builder::tests::dist::test_exclude ... ok
test builder::tests::dist::test_with_no_doc_stage0 ... ok

failures:
failures:

---- builder::tests::dist::test_docs stdout ----
thread 'main' panicked at 'command did not execute successfully: "git" "rev-list" "--count" "--merges" "refs/remotes/origin/master..HEAD"
expected success, got: exit status: 128', src/bootstrap/lib.rs:1167:21


failures:
    builder::tests::dist::test_docs
