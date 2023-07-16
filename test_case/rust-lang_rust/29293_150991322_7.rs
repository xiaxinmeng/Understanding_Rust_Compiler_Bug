
[282001.448603] grsec: exec of /bin/bash (/bin/bash -c exec /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV ) by /bin/bash[gdb:31020] uid/euid:1000/1000 gid/egid:1000/1000, parent /usr/bin/gdb[gdb:30613] uid/euid:1000/1000 gid/egid:1000/1000

[282001.454206] grsec: exec of /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc (/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV ) by /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc[bash:31020] uid/euid:1000/1000 gid/egid:1000/1000, parent /usr/bin/gdb[gdb:30613] uid/euid:1000/1000 gid/egid:1000/1000

[282001.472263] PAX: execution attempt in: <anonymous mapping>, 351e56b2000-351e56b3000 351e56b2000
[282001.472282] PAX: terminating task: /usr/bin/gdb(gdb):31031, uid/euid: 1000/1000, PC: 00000351e56b2000, SP: 0000038f2575db10
[282001.472294] PAX: bytes at PC: cc 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
[282001.472322] PAX: bytes at SP-8: 00000351e56b2000 0000038f2575db50 62869905ee6e2300 0000000000000000 0000006fd7b21760 0000006fd74918a0 000000000000792c 0000006fd8e80270 0000006fd8dce0d0 0000038f2575dbe0 0000006fd72f4409 
