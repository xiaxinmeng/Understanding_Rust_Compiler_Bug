
(lldb) thread backtrace
* thread #1, stop reason = signal SIGABRT
  * frame #0: 0x000000018be3fcec libsystem_kernel.dylib`__pthread_kill + 8
    frame #1: 0x000000018be70c24 libsystem_pthread.dylib`pthread_kill + 292
    frame #2: 0x000000018bdb8864 libsystem_c.dylib`abort + 104
    frame #3: 0x00000001001bf054 librustc-dev_rt.asan.dylib`::Abort() at sanitizer_posix_libcdep.cpp:155:3 [opt]
    frame #4: 0x00000001001ab200 librustc-dev_rt.asan.dylib`::ReserveShadowMemoryRange() at asan_shadow_setup.cpp:38:5 [opt]
    frame #5: 0x00000001001ab27c librustc-dev_rt.asan.dylib`::InitializeShadowMemory() at asan_shadow_setup.cpp:129:7 [opt]
    frame #6: 0x00000001001aa6f0 librustc-dev_rt.asan.dylib`::AsanInitInternal() at asan_rtl.cpp:463:3 [opt]
    frame #7: 0x00000001001aa5a0 librustc-dev_rt.asan.dylib`::AsanInitFromRtl() at asan_rtl.cpp:537:3 [opt] [artificial]
    frame #8: 0x00000001001a1fa0 librustc-dev_rt.asan.dylib`::wrap_malloc_default_zone() at sanitizer_malloc_mac.inc:86:3 [opt]
    frame #9: 0x000000018bca48c0 libsystem_malloc.dylib`__malloc_init + 896
    frame #10: 0x00000001954c27c4 libSystem.B.dylib`libSystem_initializer + 184
    frame #11: 0x000000010007f90c dyld`ImageLoaderMachO::doModInitFunctions(ImageLoader::LinkContext const&) + 868
    frame #12: 0x000000010007fb94 dyld`ImageLoaderMachO::doInitialization(ImageLoader::LinkContext const&) + 56
    frame #13: 0x000000010007984c dyld`ImageLoader::recursiveInitialization(ImageLoader::LinkContext const&, unsigned int, char const*, ImageLoader::InitializerTimingList&, ImageLoader::UninitedUpwards&) + 620
    frame #14: 0x0000000100079794 dyld`ImageLoader::recursiveInitialization(ImageLoader::LinkContext const&, unsigned int, char const*, ImageLoader::InitializerTimingList&, ImageLoader::UninitedUpwards&) + 436
    frame #15: 0x0000000100079794 dyld`ImageLoader::recursiveInitialization(ImageLoader::LinkContext const&, unsigned int, char const*, ImageLoader::InitializerTimingList&, ImageLoader::UninitedUpwards&) + 436
    frame #16: 0x0000000100079794 dyld`ImageLoader::recursiveInitialization(ImageLoader::LinkContext const&, unsigned int, char const*, ImageLoader::InitializerTimingList&, ImageLoader::UninitedUpwards&) + 436
    frame #17: 0x0000000100077300 dyld`ImageLoader::processInitializers(ImageLoader::LinkContext const&, unsigned int, ImageLoader::InitializerTimingList&, ImageLoader::UninitedUpwards&) + 192
    frame #18: 0x00000001000773cc dyld`ImageLoader::runInitializers(ImageLoader::LinkContext const&, ImageLoader::InitializerTimingList&) + 96
    frame #19: 0x000000010006284c dyld`dyld::initializeMainExecutable() + 220
    frame #20: 0x0000000100068b98 dyld`dyld::_main(macho_header const*, unsigned long, int, char const**, char const**, char const**, unsigned long*) + 7388
    frame #21: 0x0000000100061258 dyld`dyldbootstrap::start(dyld3::MachOLoaded const*, int, char const**, dyld3::MachOLoaded const*, unsigned long*) + 476
    frame #22: 0x0000000100061038 dyld`_dyld_start + 56
