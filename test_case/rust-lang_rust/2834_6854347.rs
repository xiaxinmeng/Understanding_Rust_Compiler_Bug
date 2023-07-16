
==18505== Thread 3:
==18505== Use of uninitialised value of size 8
==18505==    at 0x3FD749: shape::data<shape::log, shape::ptr>::walk_tag1(shape::tag_info&) (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505==    by 0x3FD34A: shape::ctxt<shape::data<shape::log, shape::ptr> >::walk_tag0() (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505==    by 0x3FEEF7: shape::ctxt<shape::data<shape::log, shape::ptr> >::walk() (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505==    by 0x3FB9D3: shape::log::walk_res2(shape::rust_fn const*, unsigned char const*) (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505==    by 0x3FEFA7: shape::ctxt<shape::data<shape::log, shape::ptr> >::walk() (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505==    by 0x3FB851: shape_log_str (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505==    by 0x5CA49: shape_log_str__c_stack_shim (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/libcore-d27e4777a53c3e50-0.2.dylib)
==18505==    by 0x4038FB: __morestack (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505== 
==18505== Use of uninitialised value of size 8
==18505==    at 0x3FD77D: shape::data<shape::log, shape::ptr>::walk_tag1(shape::tag_info&) (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505==    by 0x3FD34A: shape::ctxt<shape::data<shape::log, shape::ptr> >::walk_tag0() (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505==    by 0x3FEEF7: shape::ctxt<shape::data<shape::log, shape::ptr> >::walk() (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505==    by 0x3FB9D3: shape::log::walk_res2(shape::rust_fn const*, unsigned char const*) (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505==    by 0x3FEFA7: shape::ctxt<shape::data<shape::log, shape::ptr> >::walk() (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505==    by 0x3FB851: shape_log_str (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505==    by 0x5CA49: shape_log_str__c_stack_shim (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/libcore-d27e4777a53c3e50-0.2.dylib)
==18505==    by 0x4038FB: __morestack (in /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/librustrt.dylib)
==18505== 
rust: ~"res(some(1))"
==18505== Warning: client switching stacks?  SP change: 0x100076370 --> 0xb0021e70
==18505==          to suppress, use: --max-stackframe=1342522624 or greater
==18505== 
==18505== HEAP SUMMARY:
==18505==     in use at exit: 227,990 bytes in 989 blocks
==18505==   total heap usage: 1,596 allocs, 607 frees, 1,414,871 bytes allocated
==18505== 
==18505== LEAK SUMMARY:
==18505==    definitely lost: 0 bytes in 0 blocks
==18505==    indirectly lost: 0 bytes in 0 blocks
==18505==      possibly lost: 0 bytes in 0 blocks
==18505==    still reachable: 227,990 bytes in 989 blocks
==18505==         suppressed: 0 bytes in 0 blocks
==18505== Rerun with --leak-check=full to see details of leaked memory
==18505== 
==18505== For counts of detected and suppressed errors, rerun with: -v
==18505== Use --track-origins=yes to see where uninitialised values come from
==18505== ERROR SUMMARY: 2 errors from 2 contexts (suppressed: 120 from 8)
