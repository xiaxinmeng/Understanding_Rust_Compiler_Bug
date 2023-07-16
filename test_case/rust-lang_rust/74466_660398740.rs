
2020-07-18T01:04:45.6911067Z 
2020-07-18T01:04:45.6911563Z Breakpoint 1, pretty_std_collections::main::h1ef1d7ce5967b6a7 () at /checkout/obj/build/tmp/distcheck/src/test/debuginfo/pretty-std-collections.rs:140
2020-07-18T01:04:45.6911702Z 140	    zzz(); // #break
2020-07-18T01:04:45.6911819Z $1 = {map = {root = {<No data fields>}, length = 15}}
2020-07-18T01:04:45.6911942Z $2 = {map = {root = {<No data fields>}, length = 0}}
2020-07-18T01:04:45.6912060Z $3 = {root = {<No data fields>}, length = 15}
2020-07-18T01:04:45.6912175Z $4 = {root = {<No data fields>}, length = 0}
2020-07-18T01:04:45.6912289Z $5 = {root = {<No data fields>}, length = 2}
2020-07-18T01:04:45.6912402Z $6 = {root = {<No data fields>}, length = 15}
2020-07-18T01:04:45.6912622Z $7 = {tail = 0, head = 3, buf = {ptr = {pointer = 0x5555557a15e0, _marker = {<No data fields>}}, cap = 8, alloc = {<No data fields>}}}
2020-07-18T01:04:45.6912805Z $8 = {tail = 1, head = 0, buf = {ptr = {pointer = 0x5555557a1610, _marker = {<No data fields>}}, cap = 8, alloc = {<No data fields>}}}
2020-07-18T01:04:45.6912981Z $9 = {base = {hash_builder = {__0 = {<No data fields>}}, table = {bucket_mask = 7, ctrl = {pointer = 0x5555557a16e0 "\377"}, data = {pointer = 0x5555557a16f8}, growth_left = 3, items = 4, marker = {<No data fields>}}}}
2020-07-18T01:04:45.6913168Z $10 = {map = {base = {hash_builder = {__0 = {<No data fields>}}, table = {bucket_mask = 7, ctrl = {pointer = 0x5555557a1680 "\377"}, data = {pointer = 0x5555557a1698}, growth_left = 3, items = 4, marker = {<No data fields>}}}}}
2020-07-18T01:04:45.6913310Z A debugging session is active.
2020-07-18T01:04:45.6913380Z 
2020-07-18T01:04:45.6913483Z 	Inferior 1 [process 36447] will be killed.
2020-07-18T01:04:45.6913561Z 
2020-07-18T01:04:45.6913668Z Quit anyway? (y or n) [answered Y; input not from terminal]
2020-07-18T01:04:45.6913728Z 
2020-07-18T01:04:45.6913985Z ------------------------------------------
2020-07-18T01:04:45.6914095Z stderr:
2020-07-18T01:04:45.6914313Z ------------------------------------------
2020-07-18T01:04:45.6914389Z 
2020-07-18T01:04:45.6914604Z ------------------------------------------
2020-07-18T01:04:45.6914675Z 
2020-07-18T01:04:45.6914719Z 
2020-07-18T01:04:45.6914777Z 
2020-07-18T01:04:45.6914873Z failures:
2020-07-18T01:04:45.6915103Z     [debuginfo-gdb] debuginfo/function-call.rs
2020-07-18T01:04:45.6915338Z     [debuginfo-gdb] debuginfo/pretty-huge-vec.rs
2020-07-18T01:04:45.6915581Z     [debuginfo-gdb] debuginfo/pretty-std-collections.rs
2020-07-18T01:04:45.6915829Z     [debuginfo-gdb] debuginfo/pretty-uninitialized-vec.rs
2020-07-18T01:04:45.6915909Z 
2020-07-18T01:04:45.6916176Z test result: [31mFAILED(B[m. 71 passed; 4 failed; 41 ignored; 0 measured; 0 filtered out
2020-07-18T01:04:45.6916323Z 
