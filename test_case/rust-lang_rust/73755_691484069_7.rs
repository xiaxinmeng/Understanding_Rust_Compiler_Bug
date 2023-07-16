
test_wasm on  master [?] is 📦 v0.1.0 via 🦀 v1.46.0
❯ wasm-ld lib.o -o libtest.wasm --shared

test_wasm on  master [?] is 📦 v0.1.0 via 🦀 v1.46.0 took 5s
❯ wasm-objdump -d libtest.wasm

libtest.wasm:   file format wasm 0x1
0000021: error: invalid function result count 2, only 1 bytes left in section

Code Disassembly:

0000a7 func[0] <__wasm_call_ctors>:
 0000a8: 10 01                      | call 1 <__wasm_apply_relocs>
 0000aa: 0b                         | end
0000ac func[1] <__wasm_apply_relocs>:
 0000ad: 0b                         | end
0000af func[2] <magic>:
 0000b0: 20 01                      | local.get 1
 0000b2: 20 00                      | local.get 0
 0000b4: 6a                         | i32.add
 0000b5: 20 00                      | local.get 0
 0000b7: 20 01                      | local.get 1
 0000b9: 6b                         | i32.sub
 0000ba: 0b                         | end
