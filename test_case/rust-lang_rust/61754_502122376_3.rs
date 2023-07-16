
$ cargo clean; time cargo test --lib
...
    Finished dev [unoptimized + debuginfo] target(s) in 31.62s
     Running target/debug/deps/conch_runtime-403b4e0e8e40cfbe

running 24 tests
test env::args::tests::test_args ... ok
test env::args::tests::test_name ... ok
test env::args::tests::test_set_args ... ok
test env::args::tests::test_shift_args ... ok
test env::args::tests::test_sub_env_no_needless_clone ... ok
test env::fd::tests::test_sub_env_no_needless_clone ... ok
test env::fd::tests::test_set_get_and_close_file_desc ... ok
test env::fd::tests::test_set_and_closefile_desc_in_child_env_should_not_affect_parent ... ok
test env::func::tests::test_set_and_unset_function_in_child_should_not_affect_parent ... ok
test env::func::tests::test_set_function_in_parent_visible_in_child ... ok
test env::func::tests::test_set_get_unset_function ... ok
test env::func::tests::test_sub_env_no_needless_clone ... ok
test env::last_status::tests::test_set_last_status_in_child_env_should_not_affect_parent ... ok
test env::last_status::tests::test_env_set_and_get_last_status ... ok
test env::var::tests::test_env_vars ... ok
test env::var::tests::test_set_get_unset_exported_var ... ok
test env::var::tests::test_get_env_vars_visible_in_parent_and_child ... ok
test env::var::tests::test_sub_env_no_needless_clone ... ok
test env::var::tests::test_set_var_in_child_env_should_not_affect_parent ... ok
test env::var::tests::test_set_get_unset_var ... ok
test error::tests::ensure_runtime_errors_are_send_and_sync ... ok
test io::tests::ensure_file_desc_is_send_and_sync ... ok
test env::env_impl::tests::test_env_is_interactive ... ok
test io::pipe::tests::smoke ... ok

test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


real	0m31.642s
user	2m14.046s
sys	0m6.977s

//oh my gawd! wha?!! somebody pinch me

$ rustv
!! master-installed (default)
!! Executing '/home/user/.cargo/bin/rustc' in pwd='/home/user' with args(1): '-vV'
rustc 1.37.0-dev (e699ea096 2019-06-14)
binary: rustc
commit-hash: e699ea096fcc2fc9ce8e8bcf884e11496a31cc9f
commit-date: 2019-06-14
host: x86_64-unknown-linux-gnu
release: 1.37.0-dev
LLVM version: 8.0
!! master-installed (default)
!! Executing '/home/user/build/2nonpkgs/rust.stuff/cargo/cargo//target/release//cargo' in pwd='/home/user' with args(1): '-vV'
cargo 1.37.0-dev (fa05862c 2019-06-13)
release: 1.37.0
commit-hash: fa05862cd0c6b899b801fda0f256ac5b9bae69d9
commit-date: 2019-06-13
