
(lldb) bt
* thread #1, queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=1, address=0x0)
  * frame #0: 0x0000000100001864 1`core::ptr::drop_in_place<1::X>((null)=0x0000000000000000) at ptr.rs:61 [opt]
    frame #1: 0x0000000100001831 1`core::ptr::drop_in_place<alloc::boxed::Box<1::X>>((null)=0x00007fff5fbff610) at ptr.rs:61 [opt]
    frame #2: 0x000000010000189b 1`core::ptr::drop_in_place<1::E>((null)=<unavailable>) at ptr.rs:61 [opt]
    frame #3: 0x0000000100001b95 1`_ZN111gE(pt=<unavailable>) at 1.rs:47 [opt]
    frame #4: 0x0000000100001981 1`_ZN114mainE at 1.rs:4 [opt]
    frame #5: 0x0000000100008ccc 1`panic_abort::__rust_maybe_catch_panic at lib.rs:40 [opt]
    frame #6: 0x00000001000089e2 1`std::rt::lang_start [inlined] std::panicking::try<(),fn()> at panicking.rs:433 [opt]
    frame #7: 0x00000001000089b3 1`std::rt::lang_start [inlined] std::panic::catch_unwind<fn(),()> at panic.rs:361 [opt]
    frame #8: 0x00000001000089b3 1`std::rt::lang_start at rt.rs:57 [opt]
    frame #9: 0x00007fffc988b235 libdyld.dylib`start + 1
(lldb) fr s 3
frame #3: 0x0000000100001b95 1`_ZN111gE(pt=<unavailable>) at 1.rs:47 [opt]
   44  	            _ => {
   45  	                Err(y.unwrap())? // <-- must use `?`, return Err won't trigger segfault
   46  	            }
-> 47  	        }
   48  	    }
   49  	}
