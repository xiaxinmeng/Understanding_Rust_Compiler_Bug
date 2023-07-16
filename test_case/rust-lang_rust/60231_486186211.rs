
$ wasm-dis foo/lib-1oxx6vmtp942d/s-fbld28zq8j-122tlgg-2xj38ws0mwqgo/x73358tcv87tyh4.o
(module
 (type $0 (func))
 (type $1 (func (param i32)))
 (type $2 (func (param i32 i32) (result i32)))
 (import "env" "__linear_memory" (memory $0 1))
 (import "env" "__indirect_function_table" (table 0 anyfunc))
 (import "env" "_ZN4core3str21_$LT$impl$u20$str$GT$6as_ptr17h490791c777cb2b36E" (func $fimport$2 (param i32 i32) (result i32)))
 (import "import" "foo" (func $fimport$3 (param i32)))
 (data (i32.const 0) "Hi")
…
 (func $2 (; 4 ;) (type $0)
…
  (set_local $var$0
   (i32.const 0)
  )
…
 )
 ;; custom section "linking", size 117
 ;; custom section "reloc.CODE", size 12
)
