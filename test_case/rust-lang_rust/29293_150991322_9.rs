 bash
[284431.059807] grsec: exec of /usr/bin/gdb (gdb --args /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV ) by /usr/bin/gdb[bash:20489] uid/euid:1000/1000 gid/egid:1000/1000, parent /bin/bash[bash:30555] uid/euid:1000/1000 gid/egid:1000/1000
[284431.081685] grsec: exec of /usr/bin/iconv (iconv -l ) by /usr/bin/iconv[gdb:20490] uid/euid:1000/1000 gid/egid:1000/1000, parent /usr/bin/gdb[gdb:20489] uid/euid:1000/1000 gid/egid:1000/1000
[284431.901487] grsec: exec of /bin/bash (/bin/bash -c exec /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV ) by /bin/bash[gdb:20491] uid/euid:1000/1000 gid/egid:1000/1000, parent /usr/bin/gdb[gdb:20489] uid/euid:1000/1000 gid/egid:1000/1000
[284431.909694] grsec: exec of /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc (/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV ) by /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc[bash:20491] uid/euid:1000/1000 gid/egid:1000/1000, parent /usr/bin/gdb[gdb:20489] uid/euid:1000/1000 gid/egid:1000/1000
[284431.911916] grsec: Segmentation fault occurred at            (nil) in /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc[bash:20491] uid/euid:1000/1000 gid/egid:1000/1000, parent /usr/bin/gdb[gdb:20489] uid/euid:1000/1000 gid/egid:1000/1000

...
[284447.041691] grsec: exec of /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc (/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV ) by /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc[bash:20514] uid/euid:1000/1000 gid/egid:1000/1000, parent /usr/bin/gdb[gdb:20512] uid/euid:1000/1000 gid/egid:1000/1000
[284447.065900] PAX: execution attempt in: <anonymous mapping>, 383691f5000-383691f6000 383691f5000
[284447.065917] PAX: terminating task: /usr/bin/gdb(gdb):20517, uid/euid: 1000/1000, PC: 00000383691f5000, SP: 000003dc07db2ce0
[284447.065927] PAX: bytes at PC: cc 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
[284447.065955] PAX: bytes at SP-8: 00000383691f5000 000003dc07db2d20 bbee2a0d7dc77d00 0000000000000000 0000007af2533760 0000007af1ea38a0 0000000000005022 0000007af5893a80 0000007af57d31e0 000003dc07db2db0 0000007af1d06409 
...

[284483.000550] grsec: Segmentation fault occurred at            (nil) in /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc[bash:20543] uid/euid:1000/1000 gid/egid:1000/1000, parent /usr/bin/gdb[gdb:20541] uid/euid:1000/1000 gid/egid:1000/1000
...

[284496.186188] grsec: Segmentation fault occurred at            (nil) in /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc[bash:20566] uid/euid:1000/1000 gid/egid:1000/1000, parent /usr/bin/gdb[gdb:20564] uid/euid:1000/1000 gid/egid:1000/1000
...
