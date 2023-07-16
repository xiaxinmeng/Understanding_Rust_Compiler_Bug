
❯ wasm-objdump -d target/wasm32-unknown-unknown/release/test_wasm.wasm

test_wasm.wasm: file format wasm 0x1
0000010: error: invalid function result count 2, only 1 bytes left in section

Code Disassembly:

00006f func[0] <magic>:
 000070: 20 01                      | local.get 1
 000072: 20 00                      | local.get 0
 000074: 6a                         | i32.add
 000075: 20 00                      | local.get 0
 000077: 20 01                      | local.get 1
 000079: 6b                         | i32.sub
 00007a: 0b                         | end

test_wasm on  master [?] is 📦 v0.1.0 via 🦀 v1.46.0
❯ wasm-objdump -x target/wasm32-unknown-unknown/release/test_wasm.wasm

test_wasm.wasm: file format wasm 0x1
0000010: error: invalid function result count 2, only 1 bytes left in section

Section Details:

Type[1]:
Function[1]:
 - func[0] sig=0 <magic>
Table[1]:
 - table[0] type=funcref initial=1 max=1
Memory[1]:
 - memory[0] pages: initial=16
Global[3]:
 - global[0] i32 mutable=1 - init i32=1048576
 - global[1] i32 mutable=0 <__data_end> - init i32=1048576
 - global[2] i32 mutable=0 <__heap_base> - init i32=1048576
Export[4]:
 - memory[0] -> "memory"
 - func[0] <magic> -> "magic"
 - global[1] -> "__data_end"
 - global[2] -> "__heap_base"
Code[1]:
 - func[0] size=12 <magic>
Custom:
 - name: ".debug_info"
Custom:
 - name: ".debug_pubtypes"
Custom:
 - name: ".debug_ranges"
Custom:
 - name: ".debug_aranges"
Custom:
 - name: ".debug_abbrev"
Custom:
 - name: ".debug_line"
Custom:
 - name: ".debug_str"
Custom:
 - name: ".debug_pubnames"
Custom:
 - name: "name"
 - func[0] <magic>
Custom:
 - name: "producers"
Custom:
 - name: "target_features"
