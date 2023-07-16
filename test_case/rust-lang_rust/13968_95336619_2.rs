 shell
$ for v in version{1a,2a,4a,5a,6a,1b,2b,3b,4b,5b,3a} ; do echo $v; rustc -L. --cfg $v m.rs && ./m ; done
version1a
n::C is 3
version2a
n::C is 3
version4a
n::C is 3
version5a
n::C is 3
version6a
n::C is 3
version1b
n::C is 3
version2b
n::C is 3
version3b
n::C is 3
version4b
n::C is 3
version5b
n::C is 3
version3a
m.rs:5:23: 5:24 error: import `n` conflicts with imported crate in this module (maybe you meant `use n::*`?) [E0254]
m.rs:5 #[cfg(version3a)] use n;
                             ^
error: aborting due to previous error
