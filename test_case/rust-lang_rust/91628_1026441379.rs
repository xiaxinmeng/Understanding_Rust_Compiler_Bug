
    $ wasm-emscripten-finalize --minimize-wasm-changes --dyncalls-i64 target/wasm32-unknown-emscripten/release/deps/rust_emscripten_bug.wasm -o target/wasm32-unknown-emscripten/release/deps/rust_emscripten_bug.wasm  --detect-features --debug

<snip>

readExpression seeing 16
zz node: Call
<==
getInt8: 128 (at 23275)
getInt8: 128 (at 23276)
getInt8: 128 (at 23277)
getInt8: 128 (at 23278)
getInt8: 0 (at 23279)
getU32LEB: 0 ==>
== popExpression
== popExpression
== popExpression
== popExpression
== popExpression
== popExpression
== popExpression
== popExpression
[parse exception: attempted pop from empty stack / beyond block start boundary at 23280 (at 0:23280)]
Fatal: error in parsing input
