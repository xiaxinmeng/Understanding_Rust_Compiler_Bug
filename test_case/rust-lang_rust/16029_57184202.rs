
# tmp.crash is the above example including "extern crate rustc"
[5196 john ~]% nm tmp.crash| grep 'rt.*init'

# tmp.nocrash is the example without "extern crate libc"
[5197 john ~]% nm tmp.nocrash| grep 'rt.*init'
0000000100054690 T __ZN2io25IoFactory.rtio..IoFactory10timer_init20hc9a34fb8d223cf2672dE
0000000100061460 T __ZN2rt4init20h9bb1881593d90783tirE
