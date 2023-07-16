
src/libstd/sys/cloudabi/condvar.rs
82:        let mut event: abi::event = mem::uninitialized();
83:        let mut nevents: usize = mem::uninitialized();
134:        let mut events: [abi::event; 2] = mem::uninitialized();
135:        let mut nevents: usize = mem::uninitialized();

src/libstd/sys/cloudabi/mod.rs
62:        let mut v = mem::uninitialized();

src/libstd/sys/cloudabi/time.rs
21:            let mut t = mem::uninitialized();
62:            let mut t = mem::uninitialized();

src/libstd/sys/sgx/mod.rs
143:            let mut ret: u64 = crate::mem::uninitialized();

src/libstd/sys/cloudabi/mutex.rs
56:        mem::uninitialized()
103:            let mut event: abi::event = mem::uninitialized();
104:            let mut nevents: usize = mem::uninitialized();

src/libstd/sys/cloudabi/thread.rs
75:            let mut event: abi::event = mem::uninitialized();
76:            let mut nevents: usize = mem::uninitialized();

src/libstd/sys/cloudabi/rwlock.rs
76:            let mut event: abi::event = mem::uninitialized();
77:            let mut nevents: usize = mem::uninitialized();
185:            let mut event: abi::event = mem::uninitialized();
186:            let mut nevents: usize = mem::uninitialized();

src/libstd/sys/cloudabi/abi/cloudabi.rs
1096:    let obj: auxv = core::mem::uninitialized();
1109:    let obj: auxv = core::mem::uninitialized();
1130:    let obj: ciovec = core::mem::uninitialized();
1142:    let obj: ciovec = core::mem::uninitialized();
1170:    let obj: dirent = core::mem::uninitialized();
1234:    let obj: event = core::mem::uninitialized();
1268:    let obj: fdstat = core::mem::uninitialized();
1305:    let obj: filestat = core::mem::uninitialized();
1331:    let obj: iovec = core::mem::uninitialized();
1343:    let obj: iovec = core::mem::uninitialized();
1366:    let obj: lookup = core::mem::uninitialized();
1400:    let obj: recv_in = core::mem::uninitialized();
1415:    let obj: recv_in = core::mem::uninitialized();
1444:    let obj: recv_out = core::mem::uninitialized();
1458:    let obj: recv_out = core::mem::uninitialized();
1486:    let obj: send_in = core::mem::uninitialized();
1501:    let obj: send_in = core::mem::uninitialized();
1524:    let obj: send_out = core::mem::uninitialized();
1535:    let obj: send_out = core::mem::uninitialized();
1653:    let obj: subscription = core::mem::uninitialized();
1680:    let obj: subscription = core::mem::uninitialized();
1734:    let obj: tcb = core::mem::uninitialized();
1745:    let obj: tcb = core::mem::uninitialized();
1779:    let obj: threadattr = core::mem::uninitialized();
1793:    let obj: threadattr = core::mem::uninitialized();
