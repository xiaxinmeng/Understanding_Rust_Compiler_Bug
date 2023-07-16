
(gdb) p some_bool::B
$1 = true
(gdb) p some_bool::C
No symbol "C" in namespace "some_bool".
(gdb) p some_bool::nested::C
No type "nested" within class or namespace "some_bool".
