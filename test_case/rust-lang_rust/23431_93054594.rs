
22:42:34 <doener_> Aaron: ah, the load from test.en is indeed qualified with
                   the wrong alignment
22:44:11 <doener_> Aaron: the Test struct has an aligment on 8, and the enum
                   has an offset of 2 from that. So it's aligned on 2. The load
                   from the i32* was inferred to load from an address that has
                   an alignment of 4, because rustc doesn't specify an
                   alignment for loads/store at all, llvm used the "natural"
                   alignment for that type
