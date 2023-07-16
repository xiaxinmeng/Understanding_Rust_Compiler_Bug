
#0  0x00007ffff5ee8690 in rust_panic () from /usr/local/lib/librustrt-4e7c5e5c.so
#1  0x00007ffff5ee8e6a in unwind::begin_unwind_inner::h043457e0ce41796ddSd () from /usr/local/lib/librustrt-4e7c5e5c.so
#2  0x00007ffff5ee8b29 in unwind::begin_unwind_fmt::h33cb99eebdc73181FPd () from /usr/local/lib/librustrt-4e7c5e5c.so
#3  0x00007ffff621f6ad in rand::task_rng::h87e31a49ab53b7b4Gib () from /usr/local/lib/libstd-4e7c5e5c.so
#4  0x00007ffff65fc34b in hash::RandomSipHasher::new::h7ac2f0f050fe1602c5f () from ./libplugin.so
#5  0x00007ffff65fc2e3 in collections::hashmap::map::HashMap$LT$K$C$$x20V$C$$x20RandomSipHasher$GT$::new::h6355596007518334254 () from ./libplugin.so
#6  0x00007ffff65fc1e9 in init () from ./libplugin.so
#7  0x000055555555e44f in main::h77f94290d38a5ccegaa ()
#8  0x0000555555594d3d in start::closure.8528 ()
#9  0x00005555555acbcc in rust_try_inner ()
#10 0x00005555555acbb6 in rust_try ()
#11 0x00005555555aa233 in unwind::try::hac84dac916339a9cxGd ()
#12 0x00005555555aa10c in task::Task::run::h3855d187b4c0788fnMc ()
#13 0x0000555555594b7f in start::h6e41679d4b8806beUhe ()
#14 0x00005555555949b6 in lang_start::h1776e07af04107b5dhe ()
#15 0x000055555555e4df in main ()
