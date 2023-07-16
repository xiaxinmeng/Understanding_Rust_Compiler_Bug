text
> cargo +nightly test
   Compiling tokio-jsonrpc v0.1.0 (C:\Users\Steve Klabnik\tmp\padgrid\tokio-jsonrpc)
    Finished dev [unoptimized + debuginfo] target(s) in 4.23s
     Running target\debug\deps\tokio_jsonrpc-80ee290b51107abb.exe

running 5 tests
test codec::tests::encode ... ok
test codec::tests::decode ... ok
test message::tests::broken ... ok
test message::tests::message_serde ... ok
test message::tests::constructors ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\debug\deps\endpoint-7bde32359485fe52.exe

running 8 tests
test notification ... ok
test rpc_answer ... ok
test wrong_method ... ok
test client_only ... ok
test parallel ... ok
test delayed ... ok
test seq ... FAILED
test timeout ... ok

failures:

---- seq stdout ----
Terminating
MSG: Ok(Request(Request { jsonrpc: Version, method: "timeout", params: Some(Array([Number(PosInt(0)), Number(PosInt(500000000))])), id: String("381c9496-8fdb-48fc-9a22-9ff13e40db5b") }))
MSG: Ok(Request(Request { jsonrpc: Version, method: "wrong", params: None, id: String("2ec80644-b94a-44c7-bdc2-9442a31fe0ca") }))
Terminating
Dropping
Cleanup
MSG: Ok(Response(Response { jsonrpc: Version, result: Ok(Bool(true)), id: String("381c9496-8fdb-48fc-9a22-9ff13e40db5b") }))
MSG: Ok(Response(Response { jsonrpc: Version, result: Err(RPCError { code: -32601, message: "Method not found", data: Some(String("wrong")) }), id: String("2ec80644-b94a-44c7-bdc2-9442a31fe0ca") }))
Answer receivedAnswer receivedDropping
thread 'seq' panicked at 'assertion failed: !first_finished.get()', tests\endpoint.rs:304:25
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    seq

test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
