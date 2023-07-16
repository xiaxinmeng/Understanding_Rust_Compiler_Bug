
$ rustc --version
rustc 1.31.0 (abe02cefd 2018-12-04)
$
$
$ cat raii.rs 
fn create_box() {
  let _box = Box::new(1u8);
}

fn main() {
  let _box_one = Box::new(10u8);

  {
    let _box_two = Box::new(20u8);
  }

  for _ in 0u16..1_000 {
    create_box();
  }
}
$
$
$ rustc raii.rs && valgrind --leak-check=full --show-leak-kinds=all ./raii
==43885== Memcheck, a memory error detector
==43885== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==43885== Using Valgrind-3.14.0 and LibVEX; rerun with -h for copyright info
==43885== Command: ./raii
==43885== 
--43885-- run: /usr/bin/dsymutil "./raii"
==43885== 
==43885== HEAP SUMMARY:
==43885==     in use at exit: 18,719 bytes in 162 blocks
==43885==   total heap usage: 184 allocs, 22 frees, 27,191 bytes allocated
==43885== 
==43885== 24 bytes in 1 blocks are still reachable in loss record 3 of 44
==43885==    at 0x1001332FE: malloc_zone_malloc (in /usr/local/Cellar/valgrind/3.14.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==43885==    by 0x100807901: NXCreateMapTableFromZone (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x1008078D3: NXCreateMapTable (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100806394: __sel_registerName(char const*, int, int) (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100806096: sel_init (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10080598C: map_images_nolock (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x1008184E0: map_images (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10008EC64: dyld::notifyBatchPartial(dyld_image_states, bool, char const* (*)(dyld_image_states, unsigned int, dyld_image_info const*), bool, bool) (in /usr/lib/dyld)
==43885==    by 0x10008EE39: dyld::registerObjCNotifiers(void (*)(unsigned int, char const* const*, mach_header const* const*), void (*)(char const*, mach_header const*), void (*)(char const*, mach_header const*)) (in /usr/lib/dyld)
==43885==    by 0x1002D071D: _dyld_objc_notify_register (in /usr/lib/system/libdyld.dylib)
==43885==    by 0x100805073: _objc_init (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10025AB34: _os_object_init (in /usr/lib/system/libdispatch.dylib)
==43885== 
==43885== 32 bytes in 1 blocks are still reachable in loss record 8 of 44
==43885==    at 0x1001332FE: malloc_zone_malloc (in /usr/local/Cellar/valgrind/3.14.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==43885==    by 0x100807A9E: NXCreateHashTableFromZone (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100807A6E: NXCreateHashTable (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100807948: NXCreateMapTableFromZone (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x1008078D3: NXCreateMapTable (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100806394: __sel_registerName(char const*, int, int) (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100806096: sel_init (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10080598C: map_images_nolock (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x1008184E0: map_images (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10008EC64: dyld::notifyBatchPartial(dyld_image_states, bool, char const* (*)(dyld_image_states, unsigned int, dyld_image_info const*), bool, bool) (in /usr/lib/dyld)
==43885==    by 0x10008EE39: dyld::registerObjCNotifiers(void (*)(unsigned int, char const* const*, mach_header const* const*), void (*)(char const*, mach_header const*), void (*)(char const*, mach_header const*)) (in /usr/lib/dyld)
==43885==    by 0x1002D071D: _dyld_objc_notify_register (in /usr/lib/system/libdyld.dylib)
==43885== 
==43885== 32 bytes in 1 blocks are still reachable in loss record 9 of 44
==43885==    at 0x1001332FE: malloc_zone_malloc (in /usr/local/Cellar/valgrind/3.14.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==43885==    by 0x100807ABF: NXCreateHashTableFromZone (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100807A6E: NXCreateHashTable (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100807948: NXCreateMapTableFromZone (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x1008078D3: NXCreateMapTable (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100806394: __sel_registerName(char const*, int, int) (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100806096: sel_init (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10080598C: map_images_nolock (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x1008184E0: map_images (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10008EC64: dyld::notifyBatchPartial(dyld_image_states, bool, char const* (*)(dyld_image_states, unsigned int, dyld_image_info const*), bool, bool) (in /usr/lib/dyld)
==43885==    by 0x10008EE39: dyld::registerObjCNotifiers(void (*)(unsigned int, char const* const*, mach_header const* const*), void (*)(char const*, mach_header const*), void (*)(char const*, mach_header const*)) (in /usr/lib/dyld)
==43885==    by 0x1002D071D: _dyld_objc_notify_register (in /usr/lib/system/libdyld.dylib)
==43885== 
==43885== 32 bytes in 1 blocks are still reachable in loss record 10 of 44
==43885==    at 0x100133086: malloc (in /usr/local/Cellar/valgrind/3.14.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==43885==    by 0x100807B83: NXCreateHashTableFromZone (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100807A6E: NXCreateHashTable (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100807948: NXCreateMapTableFromZone (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x1008078D3: NXCreateMapTable (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100806394: __sel_registerName(char const*, int, int) (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100806096: sel_init (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10080598C: map_images_nolock (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x1008184E0: map_images (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10008EC64: dyld::notifyBatchPartial(dyld_image_states, bool, char const* (*)(dyld_image_states, unsigned int, dyld_image_info const*), bool, bool) (in /usr/lib/dyld)
==43885==    by 0x10008EE39: dyld::registerObjCNotifiers(void (*)(unsigned int, char const* const*, mach_header const* const*), void (*)(char const*, mach_header const*), void (*)(char const*, mach_header const*)) (in /usr/lib/dyld)
==43885==    by 0x1002D071D: _dyld_objc_notify_register (in /usr/lib/system/libdyld.dylib)
==43885== 
==43885== 32 bytes in 1 blocks are still reachable in loss record 11 of 44
==43885==    at 0x100133086: malloc (in /usr/local/Cellar/valgrind/3.14.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==43885==    by 0x1008079AE: NXCreateMapTableFromZone (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x1008078D3: NXCreateMapTable (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100806394: __sel_registerName(char const*, int, int) (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100806096: sel_init (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10080598C: map_images_nolock (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x1008184E0: map_images (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10008EC64: dyld::notifyBatchPartial(dyld_image_states, bool, char const* (*)(dyld_image_states, unsigned int, dyld_image_info const*), bool, bool) (in /usr/lib/dyld)
==43885==    by 0x10008EE39: dyld::registerObjCNotifiers(void (*)(unsigned int, char const* const*, mach_header const* const*), void (*)(char const*, mach_header const*), void (*)(char const*, mach_header const*)) (in /usr/lib/dyld)
==43885==    by 0x1002D071D: _dyld_objc_notify_register (in /usr/lib/system/libdyld.dylib)
==43885==    by 0x100805073: _objc_init (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10025AB34: _os_object_init (in /usr/lib/system/libdispatch.dylib)
==43885== 
==43885== 48 bytes in 1 blocks are still reachable in loss record 15 of 44
==43885==    at 0x1001338AD: malloc_zone_calloc (in /usr/local/Cellar/valgrind/3.14.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==43885==    by 0x100807F24: _NXHashRehashToCapacity (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100807EA2: NXHashInsert (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100807BAD: NXCreateHashTableFromZone (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100807A6E: NXCreateHashTable (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100807948: NXCreateMapTableFromZone (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x1008078D3: NXCreateMapTable (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100806394: __sel_registerName(char const*, int, int) (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x100806096: sel_init (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10080598C: map_images_nolock (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x1008184E0: map_images (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10008EC64: dyld::notifyBatchPartial(dyld_image_states, bool, char const* (*)(dyld_image_states, unsigned int, dyld_image_info const*), bool, bool) (in /usr/lib/dyld)
==43885== 
==43885== 72 bytes in 3 blocks are possibly lost in loss record 26 of 44
==43885==    at 0x1001336EA: calloc (in /usr/local/Cellar/valgrind/3.14.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==43885==    by 0x1008057C2: map_images_nolock (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x1008184E0: map_images (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10008EC64: dyld::notifyBatchPartial(dyld_image_states, bool, char const* (*)(dyld_image_states, unsigned int, dyld_image_info const*), bool, bool) (in /usr/lib/dyld)
==43885==    by 0x10008EE39: dyld::registerObjCNotifiers(void (*)(unsigned int, char const* const*, mach_header const* const*), void (*)(char const*, mach_header const*), void (*)(char const*, mach_header const*)) (in /usr/lib/dyld)
==43885==    by 0x1002D071D: _dyld_objc_notify_register (in /usr/lib/system/libdyld.dylib)
==43885==    by 0x100805073: _objc_init (in /usr/lib/libobjc.A.dylib)
==43885==    by 0x10025AB34: _os_object_init (in /usr/lib/system/libdispatch.dylib)
==43885==    by 0x10025AB1B: libdispatch_init (in /usr/lib/system/libdispatch.dylib)
==43885==    by 0x1001419C2: libSystem_initializer (in /usr/lib/libSystem.B.dylib)
==43885==    by 0x1000A0AC5: ImageLoaderMachO::doModInitFunctions(ImageLoader::LinkContext const&) (in /usr/lib/dyld)
==43885==    by 0x1000A0CF5: ImageLoaderMachO::doInitialization(ImageLoader::LinkContext const&) (in /usr/lib/dyld)
==43885== 
==43885== 112 bytes in 1 blocks are still reachable in loss record 27 of 44
==43885==    at 0x100133086: malloc (in /usr/local/Cellar/valgrind/3.14.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==43885==    by 0x1002D0969: tlv_allocate_and_initialize_for_key (in /usr/lib/system/libdyld.dylib)
==43885==    by 0x1002D10EB: tlv_get_addr (in /usr/lib/system/libdyld.dylib)
==43885==    by 0x10000938E: std::sys_common::thread_info::THREAD_INFO::__getit (cell.rs:258)
==43885==    by 0x10000A21F: <std::thread::local::LocalKey<T>>::with (local.rs:297)
==43885==    by 0x100008599: std::rt::lang_start_internal (thread_info.rs:47)
==43885==    by 0x1000017A1: std::rt::lang_start (in ./raii)
==43885==    by 0x100001371: main (in ./raii)
==43885== 
==43885== LEAK SUMMARY:
==43885==    definitely lost: 0 bytes in 0 blocks
==43885==    indirectly lost: 0 bytes in 0 blocks
==43885==      possibly lost: 72 bytes in 3 blocks
==43885==    still reachable: 312 bytes in 7 blocks
==43885==         suppressed: 18,335 bytes in 152 blocks
==43885== 
==43885== For counts of detected and suppressed errors, rerun with: -v
==43885== ERROR SUMMARY: 1 errors from 1 contexts (suppressed: 9 from 9)
