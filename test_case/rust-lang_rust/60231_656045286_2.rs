
$ wasm-objdump -d lib.lib.3a1fbbbh-cgu.0.rcgu.o

lib.lib.3a1fbbbh-cgu.0.rcgu.o:  file format wasm 0x1

Code Disassembly:

00008e func[1] <rust_begin_unwind>:
 00008f: 00                         | unreachable
 000090: 00                         | unreachable
 000091: 0b                         | end
000093 func[2] <rust_eh_personality>:
 000094: 0b                         | end
000096 func[3] <_start>:
 000097: 23 80 80 80 80 00          | global.get 0 <env.__memory_base>
 00009d: 41 80 80 80 80 00          | i32.const 0
 0000a3: 6a                         | i32.add
 0000a4: 10 80 80 80 80 00          | call 0 <env.foo>
 0000aa: 0b                         | end
