
$ gdb -quiet --args rustc --version
Reading symbols from rustc...(no debugging symbols found)...done.
(gdb) break main
Breakpoint 1 at 0xa10
(gdb) run
Starting program: /usr/local/bin/rustc --version
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/libthread_db.so.1".

Breakpoint 1, 0x80000a10 in main ()
(gdb) bt
#0  0x80000a10 in main ()
(gdb) list
1	/* Data for i386 version of processor capability information.
2	   Copyright (C) 2001-2013 Free Software Foundation, Inc.
3	   This file is part of the GNU C Library.
4	   Contributed by Ulrich Drepper <drepper@redhat.com>, 2001.
5	
6	   The GNU C Library is free software; you can redistribute it and/or
7	   modify it under the terms of the GNU Lesser General Public
8	   License as published by the Free Software Foundation; either
9	   version 2.1 of the License, or (at your option) any later version.
10	
(gdb)
