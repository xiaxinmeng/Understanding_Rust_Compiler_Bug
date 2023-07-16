
$ wasm-objdump -sj type wasm-ld_linked_rust.wasm 

test.wasm:	file format wasm 0x1
0000012: error: invalid function result count 2, only 1 bytes left in section

Contents of section Type:
000000a: 0260 0000 6001 7f02 7f                   .`..`....
         ^ ^       ^      ^  ^ ^
         | +-------+      |  | +-- MISSING
    vector         |      |  |
   length 2     type      |  type
            function      |  i32
                          |
              vector (return types)
                    length 2
