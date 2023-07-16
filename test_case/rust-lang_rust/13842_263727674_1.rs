text
$ for x in short medium long; do echo $x; time cargo rustc -- --cfg $x -Z no-trans; done
short
   Compiling testingtesting v0.1.0 (file:///C:/Users/steve/tmp/testingtesting)
    Finished debug [unoptimized + debuginfo] target(s) in 0.34 secs

real    0m0.583s
user    0m0.000s
sys     0m0.015s
medium
   Compiling testingtesting v0.1.0 (file:///C:/Users/steve/tmp/testingtesting)
    Finished debug [unoptimized + debuginfo] target(s) in 3.21 secs

real    0m3.427s
user    0m0.000s
sys     0m0.000s
long
   Compiling testingtesting v0.1.0 (file:///C:/Users/steve/tmp/testingtesting)
    Finished debug [unoptimized + debuginfo] target(s) in 11.4 secs

real    0m11.297s
user    0m0.000s
sys     0m0.015s
