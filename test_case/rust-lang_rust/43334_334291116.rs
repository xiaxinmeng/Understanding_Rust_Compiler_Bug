
(gdb) p a
$28 = (std::path::Path *) 0x555555599800 <str>
(gdb) p &a
$29 = (std::path::Path **) 0x7fffffffdf10
(gdb) p /x 0x7fffffffdf10 + 8
$30 = 0x7fffffffdf18
(gdb) p $ as *mut usize
$31 = (usize *) 0x7fffffffdf18
(gdb) p *$
$32 = 3
