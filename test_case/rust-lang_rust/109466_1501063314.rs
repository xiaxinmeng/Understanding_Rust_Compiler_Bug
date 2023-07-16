plain
 finished in 11.234 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 145 tests
iiiiii.......i...iii......i...i..........F.....i............iiiii....ii.........i......i  88/145

failures:

---- [debuginfo-gdb] tests/debuginfo/function-names.rs stdout ----
---- [debuginfo-gdb] tests/debuginfo/function-names.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 12001000

error: line not found in debugger output: [...]static fn function_names::generic_func::{closure#0}<i32>(*mut function_names::generic_func::{closure_env#0}<i32>);
status: exit status: 0
command: PYTHONPATH="/checkout/./src/etc" "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/function-names.gdb/function-names.debugger.script"
GNU gdb (Ubuntu 12.1-0ubuntu1~22.04) 12.1
Copyright (C) 2022 Free Software Foundation, Inc.
Copyright (C) 2022 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word".
File /checkout/tests/debuginfo/function-names.rs:
93: static fn function_names::main();
93: static fn function_names::main();
110: static fn function_names::main::{closure#0}();
114: static fn function_names::main::{generator#1}() -> core::ops::generator::GeneratorState<(), ()>;
File /checkout/tests/debuginfo/function-names.rs:
File /checkout/tests/debuginfo/function-names.rs:
196: static fn function_names::generic_func::{closure#0}<i32>();
194: static fn function_names::generic_func<i32>(i32) -> i32;
File /checkout/tests/debuginfo/function-names.rs:
File /checkout/tests/debuginfo/function-names.rs:
162: static fn function_names::GenericStruct<i32, i32>::impl_function<i32, i32>();
150: static fn function_names::Mod1::TestStruct2::impl_function();
134: static fn function_names::TestStruct1::impl_function();
164: static fn function_names::{impl#2}::impl_function::{closure#0}<i32, i32>();
File /checkout/tests/debuginfo/function-names.rs:
File /checkout/tests/debuginfo/function-names.rs:
154: static fn function_names::Mod1::{impl#1}::trait_function();
139: static fn function_names::{impl#1}::trait_function();
171: static fn function_names::{impl#3}::trait_function<i32>();
185: static fn function_names::{impl#5}::trait_function3<function_names::TestStruct1>();
190: static fn function_names::{impl#6}::trait_function<i32, 1>();
File /checkout/tests/debuginfo/function-names.rs:
File /checkout/tests/debuginfo/function-names.rs:
196: static fn function_names::generic_func::{closure#0}<i32>();
110: static fn function_names::main::{closure#0}();
164: static fn function_names::{impl#2}::impl_function::{closure#0}<i32, i32>();
File /checkout/tests/debuginfo/function-names.rs:
File /checkout/tests/debuginfo/function-names.rs:
202: static fn function_names::const_generic_fn_bool<false>();
203: static fn function_names::const_generic_fn_non_int<{CONST#ad91263f6d2dd96e}>();
204: static fn function_names::const_generic_fn_signed_int<-7>();
205: static fn function_names::const_generic_fn_unsigned_int<14>();
stderr: none



