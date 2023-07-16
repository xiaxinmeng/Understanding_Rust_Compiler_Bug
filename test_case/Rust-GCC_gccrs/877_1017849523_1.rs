
Breakpoint 6, dump_generic_node (pp=0x7fffffffd9e0, node=<type_decl 0x7ffff74672f8 Header>, spc=0, 
    flags=100663296, is_stmt=false) at ../../gccrs/gcc/tree-pretty-print.c:2285
2285	      if (DECL_IS_UNDECLARED_BUILTIN (node))
(gdb) p node
$2 = <type_decl 0x7ffff74672f8 Header>
(gdb) pt node
 <type_decl 0x7ffff74672f8 Header
    type <record_type 0x7ffff7466690 Header sizes-gimplified BLK
        size <integer_cst 0x7ffff7459648 constant 160>
        unit-size <integer_cst 0x7ffff7459618 constant 20>
        align:32 warn_if_not_align:0 symtab:0 alias-set -1 structural-equality
        fields <field_decl 0x7ffff745be40 magic type <integer_type 0x7ffff7452540 u16>
            unsigned HI (null):0:0
            size <integer_cst 0x7ffff743bdb0 constant 16>
            unit-size <integer_cst 0x7ffff743bdc8 constant 2>
            align:16 warn_if_not_align:0 offset_align 128
            offset <integer_cst 0x7ffff743bc30 constant 0>
            bit-offset <integer_cst 0x7ffff743bc78 constant 0> context <record_type 0x7ffff74665e8> chain <field_decl 0x7ffff745bed8 version>>>
    VOID (null):0:0
    align:1 warn_if_not_align:0
    result <record_type 0x7ffff74665e8 sizes-gimplified BLK size <integer_cst 0x7ffff7459648 160> unit-size <integer_cst 0x7ffff7459618 20>
        align:32 warn_if_not_align:0 symtab:0 alias-set -1 structural-equality fields <field_decl 0x7ffff745be40 magic>>>
(gdb) n
2288		  break;
(gdb) bt
#0  dump_generic_node (pp=0x7fffffffd9e0, node=<type_decl 0x7ffff74672f8 Header>, spc=0, flags=100663296, 
    is_stmt=false) at ../../gccrs/gcc/tree-pretty-print.c:2288
#1  0x000000000159245a in dump_generic_node (pp=0x7fffffffd9e0, node=<record_type 0x7ffff7466738 Header>, 
    spc=0, flags=100663296, is_stmt=false) at ../../gccrs/gcc/tree-pretty-print.c:2038
#2  0x000000000159d563 in print_declaration (pp=0x7fffffffd9e0, t=<var_decl 0x7ffff7fc5bd0 my_h>, spc=0, 
    flags=100663296) at ../../gccrs/gcc/tree-pretty-print.c:3894
#3  0x000000000104a4c7 in dump_gimple_bind (buffer=0x7fffffffd9e0, gs=0x7ffff7450040, spc=0, flags=100663296)
    at ../../gccrs/gcc/gimple-pretty-print.c:1197
#4  0x000000000104ed05 in pp_gimple_stmt_1 (buffer=0x7fffffffd9e0, gs=0x7ffff7450040, spc=0, flags=100663296)
    at ../../gccrs/gcc/gimple-pretty-print.c:2687
#5  0x000000000104747d in dump_gimple_seq (buffer=0x7fffffffd9e0, seq=0x7ffff7450040, spc=0, flags=100663296)
    at ../../gccrs/gcc/gimple-pretty-print.c:209
#6  0x0000000001047527 in print_gimple_seq (file=0x38c1f40, seq=0x7ffff7450040, spc=0, flags=100663296)
    at ../../gccrs/gcc/gimple-pretty-print.c:225
#7  0x00000000014d4308 in dump_function_to_file (fndecl=<function_decl 0x7ffff7463500 main>, file=0x38c1f40, 
    flags=100663296) at ../../gccrs/gcc/tree-cfg.c:8258
#8  0x0000000000e9a449 in dump_function (phase=5, fn=<function_decl 0x7ffff7463500 main>)
    at ../../gccrs/gcc/dumpfile.c:2072
#9  0x00000000010b3b45 in gimplify_function_tree (fndecl=<function_decl 0x7ffff7463500 main>)
    at ../../gccrs/gcc/gimplify.c:16042
#10 0x0000000000e3ce2d in cgraph_node::analyze (this=<cgraph_node * const 0x7ffff744e220 "main"/0>)
    at ../../gccrs/gcc/cgraphunit.c:670
#11 0x0000000000e3ee98 in analyze_functions (first_time=true) at ../../gccrs/gcc/cgraphunit.c:1234
#12 0x0000000000e420e8 in symbol_table::finalize_compilation_unit (this=0x7ffff743f000)
    at ../../gccrs/gcc/cgraphunit.c:2508
#13 0x000000000146ceca in compile_file () at ../../gccrs/gcc/toplev.c:483
#14 0x0000000001470024 in do_compile () at ../../gccrs/gcc/toplev.c:2233
#15 0x000000000147030d in toplev::main (this=0x7fffffffdeaa, argc=18, argv=0x7fffffffdfb8)
    at ../../gccrs/gcc/toplev.c:2372
#16 0x00000000029130de in main (argc=18, argv=0x7fffffffdfb8) at ../../gccrs/gcc/main.c:39
