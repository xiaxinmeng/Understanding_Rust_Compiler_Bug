
(gdb) info args
a = function_arg_initialization::BigStruct {a: 3, b: 4, c: 5, d: 6, e: 7, f: 8, g: 9, h: 10}
b = function_arg_initialization::BigStruct {a: 11, b: 12, c: 13, d: 14, e: 15, f: 16, g: 17, h: 18}
(gdb) info locals
No locals.
(gdb) print a
$1 = function_arg_initialization::BigStruct {a: 3, b: 4, c: 5, d: 6, e: 7, f: 8, g: 9, h: 10}
(gdb) print b
$2 = function_arg_initialization::BigStruct {a: 11, b: 12, c: 13, d: 14, e: 15, f: 16, g: 17, h: 18}
