rust
...
(gdb) bt full
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
        set = {__val = {1024, 139725835196768, 0, 1, 94323838034435, 0, 0, 139725835197048, 139725835196928, 140725488945040, 
            139725835196816, 94323838151584, 11, 94323838060395, 0, 0}}
        pid = <optimized out>
        tid = <optimized out>
        ret = <optimized out>
#1  0x00007f1474a41931 in __GI_abort () at abort.c:79
        save_stage = 1
        act = {__sigaction_handler = {sa_handler = 0x55c97ade9f03, sa_sigaction = 0x55c97ade9f03}, sa_mask = {__val = {1, 
              94323838302536, 2, 94323838197728, 1, 139725835197256, 1, 139725835197272, 94323838084789, 94323838302568, 2, 
              94323838197507, 1, 94323838302536, 2, 94323838197728}}, sa_flags = 1, sa_restorer = 0x7f1474c5db48}
        sigs = {__val = {32, 0 <repeats 15 times>}}
#2  0x000055c97adcc4a7 in std::sys::unix::abort_internal () at src/libstd/sys/unix/mod.rs:157
No locals.
#3  0x000055c97adce7a6 in std::sys_common::util::abort (args=...) at src/libstd/sys_common/util.rs:19
No locals.
#4  0x000055c97adca58f in std::sys::unix::stack_overflow::imp::signal_handler (signum=11, info=0x7f1474c5dd70, _data=<optimized out>)
    at src/libstd/sys/unix/stack_overflow.rs:99
        addr = <optimized out>
        guard = <optimized out>
#5  <signal handler called>
No locals.
#6  0x000055c97adad24e in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
#7  0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (
    self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#8  0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
#9  0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (
    self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#10 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
#11 0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (
    self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#12 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
--Type <RET> for more, q to quit, c to continue without paging--c
No locals.
#13 0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#14 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
#15 0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#16 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
#17 0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#18 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
#19 0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#20 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
...
#5056 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
#5057 0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#5058 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
#5059 0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#5060 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
#5061 0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#5062 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
#5063 0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#5064 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
#5065 0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#5066 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
#5067 0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#5068 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
#5069 0x000055c97adbc163 in blah::<impl core::clone::Clone for alloc::boxed::Box<(dyn blah::Value + 'static)>>::clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:37
No locals.
#5070 0x000055c97adad253 in <T as blah::Value>::box_clone (self=0x55c97b3340c0) at /tmp/blah/src/main.rs:26
No locals.
