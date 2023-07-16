plain
---- [rustdoc-json] src/test/rustdoc-json/fns/async_return.rs stdout ----

error: jsondoclint failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondoclint" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/async_return/async_return.json"
stdout: none
--- stderr -------------------------------
2:54324:186 not in index or paths, but refered to at '$.index["2:14227:1011"].links["`Poll::Pending`"]'
2:14562:2920 not in index or paths, but refered to at '$.index["2:14227:1011"].links["crate::task::Waker::wake"]'
2:54321:199 not in index or paths, but refered to at '$.index["2:14227:1011"].links["Poll::Ready"]'
Error: Errors validating json /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/async_return/async_return.json



failures:
