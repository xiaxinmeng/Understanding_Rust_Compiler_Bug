
/tmp $ cat foo.c 
extern int bar (int, char *);

char *s = "Foo";

int foo (int n)  { return bar(n, s); }
/tmp $ clang-9 --compile -fpic foo.c  --target=wasm32-unknown-unknown-wasm --optimize=3 --output foo.o
/tmp $ wasm-ld-9 --shared --no-entry foo.o -o foo.wasm --export=foo --gc-sections
/tmp $ wasm-dis foo.wasm 
(module
 (type $0 (func (param i32 i32) (result i32)))
 (type $1 (func))
 (type $2 (func (param i32) (result i32)))
 (import "env" "memory" (memory $0 0))
 (import "env" "__indirect_function_table" (table 0 anyfunc))
 (import "env" "__memory_base" (global $gimport$2 i32))
 (import "env" "__table_base" (global $gimport$3 i32))
 (import "env" "bar" (func $bar (param i32 i32) (result i32)))
 (data (get_global $gimport$2) "Foo\00\00\00\00\00")
 (export "foo" (func $foo))
 (func $__wasm_call_ctors (; 1 ;) (type $1)
  (call $__wasm_apply_relocs)
 )
 (func $__wasm_apply_relocs (; 2 ;) (type $1)
  (i32.store
   (i32.add
    (get_global $gimport$2)
    (i32.const 4)
   )
   (i32.add
    (get_global $gimport$2)
    (i32.const 0)
   )
  )
 )
 (func $foo (; 3 ;) (type $2) (param $var$0 i32) (result i32)
  (call $bar
   (get_local $var$0)
   (i32.load
    (i32.add
     (get_global $gimport$2)
     (i32.const 4)
    )
   )
  )
 )
 ;; custom section "dylink", size 5
 ;; custom section "producers", size 52
)
