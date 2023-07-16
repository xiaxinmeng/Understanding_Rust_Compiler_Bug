asm
        .text
        .file   "test_wasm.5nl5ejrj-cgu.0"
        .section        .text.magic,"",@
        .globl  magic
        .type   magic,@function
magic:
        .functype       magic (i32, i32) -> (i32, i32)
        local.get       1
        local.get       0
        i32.add
        local.get       0
        local.get       1
        i32.sub
        end_function
.Lfunc_end0:
        .size   magic, .Lfunc_end0-magic

        .section        .custom_section.target_features,"",@
        .int8   1
        .int8   43
        .int8   10
        .ascii  "multivalue"
        .section        .text.magic,"",@
