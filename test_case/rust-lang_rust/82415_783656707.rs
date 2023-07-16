plain
.................................................................................................... 9200/11474
.................................................................................................... 9300/11474
.................................................................................................... 9400/11474
...............................i......i............................................................. 9500/11474
......................................................................iiiiiii..iiiiii.i............. 9600/11474
.................................................................................................... 9800/11474
.................................................................................................... 9900/11474
.................................................................................................... 10000/11474
.................................................................................................... 10100/11474
---
83 }
-   --> $DIR/hexagon-enum.rs:15:1
+   --> $DIR/hexagon-enum.rs:16:1
85    |
86 LL | enum A { Apple }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

169         raw: 1,
170     },
170     },
171 }
-   --> $DIR/hexagon-enum.rs:19:1
+   --> $DIR/hexagon-enum.rs:20:1
173    |
174 LL | enum B { Banana = 255, }

257         raw: 2,
258     },
259 }
259 }
-   --> $DIR/hexagon-enum.rs:23:1
+   --> $DIR/hexagon-enum.rs:24:1
261    |
262 LL | enum C { Chaenomeles = 256, }

345         raw: 4,
346     },
347 }
347 }
-   --> $DIR/hexagon-enum.rs:27:1
+   --> $DIR/hexagon-enum.rs:28:1
349    |
350 LL | enum P { Peach = 0x1000_0000isize, }

433         raw: 4,
434     },
435 }
435 }
-   --> $DIR/hexagon-enum.rs:33:1
+   --> $DIR/hexagon-enum.rs:34:1
437    |
438 LL | enum T { Tangerine = TANGERINE as isize }


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/hexagon-enum/hexagon-enum.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/hexagon-enum/hexagon-enum.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args layout/hexagon-enum.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/hexagon-enum.rs" "-Zthreads=1" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/hexagon-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "hexagon-unknown-linux-musl" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/hexagon-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: layout_of(A) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
        memory_index: [
            0,
        ],
    },
    variants: Multiple {
        tag: Scalar {
            value: Int(
                false,
            ),
            ),
            valid_range: 0..=0,
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 0,
                    },
                    pref: Align {
                        pow2: 0,
                },
                size: Size {
                    raw: 1,
                },
---
            value: Int(
                I8,
                false,
            ),
            valid_range: 0..=0,
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            },
            scalar: Scalar {
                value: Int(
                    false,
                ),
                ),
                valid_range: 0..=0,
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 0,
        },
        pref: Align {
            pow2: 0,
    },
    size: Size {
        raw: 1,
    },
    },
}
  --> /checkout/src/test/ui/layout/hexagon-enum.rs:16:1
   |
LL | enum A { Apple } //~ ERROR: layout_of


error: layout_of(B) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
        memory_index: [
            0,
        ],
    },
    variants: Multiple {
        tag: Scalar {
            value: Int(
                false,
            ),
            ),
            valid_range: 255..=255,
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 0,
                    },
                    pref: Align {
                        pow2: 0,
                },
                size: Size {
                    raw: 1,
                },
---
            value: Int(
                I8,
                false,
            ),
            valid_range: 255..=255,
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            },
            scalar: Scalar {
                value: Int(
                    false,
                ),
                ),
                valid_range: 255..=255,
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 0,
        },
        pref: Align {
            pow2: 0,
    },
    size: Size {
        raw: 1,
    },
    },
}
  --> /checkout/src/test/ui/layout/hexagon-enum.rs:20:1
   |
LL | enum B { Banana = 255, } //~ ERROR: layout_of


error: layout_of(C) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
        memory_index: [
            0,
        ],
    },
    variants: Multiple {
        tag: Scalar {
            value: Int(
                false,
            ),
            ),
            valid_range: 256..=256,
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 1,
                    },
                    pref: Align {
                        pow2: 1,
                },
                size: Size {
                    raw: 2,
                },
---
            value: Int(
                I16,
                false,
            ),
            valid_range: 256..=256,
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            },
            scalar: Scalar {
                value: Int(
                    false,
                ),
                ),
                valid_range: 256..=256,
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 1,
        },
        pref: Align {
            pow2: 1,
    },
    size: Size {
        raw: 2,
    },
    },
}
  --> /checkout/src/test/ui/layout/hexagon-enum.rs:24:1
   |
LL | enum C { Chaenomeles = 256, } //~ ERROR: layout_of


error: layout_of(P) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
        memory_index: [
            0,
        ],
    },
    variants: Multiple {
        tag: Scalar {
            value: Int(
                false,
            ),
            ),
            valid_range: 268435456..=268435456,
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 2,
                    },
                    pref: Align {
                        pow2: 2,
                },
                size: Size {
                    raw: 4,
                },
---
            value: Int(
                I32,
                false,
            ),
            valid_range: 268435456..=268435456,
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            },
            scalar: Scalar {
                value: Int(
                    false,
                ),
                ),
                valid_range: 268435456..=268435456,
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 2,
        },
        pref: Align {
            pow2: 2,
    },
    size: Size {
        raw: 4,
    },
    },
}
  --> /checkout/src/test/ui/layout/hexagon-enum.rs:28:1
   |
LL | enum P { Peach = 0x1000_0000isize, } //~ ERROR: layout_of


error: layout_of(T) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
        memory_index: [
            0,
        ],
    },
    variants: Multiple {
        tag: Scalar {
            value: Int(
                true,
            ),
            ),
            valid_range: 2164260864..=2164260864,
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 2,
                    },
                    pref: Align {
                        pow2: 2,
                },
                size: Size {
                    raw: 4,
                },
---
            value: Int(
                I32,
                true,
            ),
            valid_range: 2164260864..=2164260864,
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            },
            scalar: Scalar {
                value: Int(
                    true,
                ),
                ),
                valid_range: 2164260864..=2164260864,
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 2,
        },
        pref: Align {
            pow2: 2,
    },
    size: Size {
        raw: 4,
    },
    },
}
  --> /checkout/src/test/ui/layout/hexagon-enum.rs:34:1
   |
LL | enum T { Tangerine = TANGERINE as isize } //~ ERROR: layout_of

error: aborting due to 5 previous errors


---
test result: FAILED. 11380 passed; 1 failed; 93 ignored; 0 measured; 0 filtered out; finished in 130.63s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:42
