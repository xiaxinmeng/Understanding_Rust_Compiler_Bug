rust
async fn f() {
    //  9: ~0.68s
    // 10: ~0.83s
    // 11: ~1.34s
    // 12: ~2.68s
    // 13: ~5.24s reached length limit -> #![type_length_limit="1228764"]
    // 14: ~7.89s reached length limit -> #![type_length_limit="2457564"]
    // 15: /playground/tools/entrypoint.sh: line 11:     7 Killed                  timeout --signal=KILL ${timeout} "$@"
    async_recursive!(9, println!("done"))
}
